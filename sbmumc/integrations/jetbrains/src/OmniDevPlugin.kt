// SBMUMC OmniDev JetBrains IDE Plugin
// Compatible with IntelliJ, WebStorm, PyCharm, etc.

package com.sbmumc.omnidev

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.LangDataKeys
import com.intellij.openapi.command.WriteCommandAction
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.Messages
import com.intellij.openapi.vfs.VirtualFile
import org.apache.http.client.methods.HttpPost
import org.apache.http.entity.StringEntity
import org.apache.http.impl.client.HttpClients
import java.net.URI

class OmniDevAction : AnAction("SBMUMC OmniDev") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val editor = event.getRequiredData(LangDataKeys.EDITOR)
        val document = editor.document
        val selection = editor.selectionModel.selectedText ?: document.text

        Messages.showInfoMessage(project, "SBMUMC OmniDev analyzing ${selection.length} characters...", "OmniDev")

        // Call OmniDev API
        val result = callOmniDevAPI("/api/v1/omnidev", mapOf(
            "action" to "analyze",
            "code" to selection
        ))

        if (result != null) {
            Messages.showInfoMessage(project, "Analysis complete: ${result.suggestions?.size ?: 0} suggestions", "OmniDev")
        }
    }
}

class RefactorAction : AnAction("SBMUMC Smart Refactor") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val editor = event.getData(LangDataKeys.EDITOR) ?: return
        val selection = editor.selectionModel.selectedText ?: return

        val suggestions = callOmniDevAPI("/api/v1/omnidev/refactor", mapOf(
            "code" to selection,
            "language" to getLanguageId(project)
        ))

        if (suggestions != null && suggestions.suggestions != null) {
            val items = suggestions.suggestions.map { it.description as String }
            val selected = Messages.showChoiceDialog(
                project,
                "Select refactoring option:",
                "SBMUMC Refactor",
                items.toTypedArray(),
                items[0],
                null
            )
            if (selected != null) {
                val suggestion = suggestions.suggestions[items.indexOf(selected)]
                WriteCommandAction.runWriteCommandAction(project) {
                    editor.document.setText(editor.document.text.replace(selection, suggestion.code))
                }
            }
        }
    }
}

class SemanticSearchAction : AnAction("SBMUMC Semantic Search") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val query = Messages.showInputDialog(project, "Enter search query:", "SBMUMC Search", Messages.getQuestionIcon())

        if (query != null) {
            val results = callOmniDevAPI("/api/v1/graph/search", mapOf("query" to query))

            if (results != null && results.isNotEmpty()) {
                val items = results.map { "${it.file}:${it.line}" }
                val selected = Messages.showChoiceDialog(
                    project, "Select result:", "Search Results", items.toTypedArray(), items[0], null
                )
                if (selected != null) {
                    val file = results[items.indexOf(selected)].file
                    openFile(project, file)
                }
            }
        }
    }
}

class CommitMessageAction : AnAction("SBMUMC Generate Commit") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return

        val diffs = getGitDiffs(project)
        val commit = callOmniDevAPI("/api/v1/commit", mapOf(
            "diffs" to diffs,
            "conventional" to true
        ))

        if (commit != null) {
            Messages.showInfoMessage(project, "Generated: ${commit.message}", "Commit Message")
            // Integrate with Git UI
        }
    }
}

class TestGenerationAction : AnAction("SBMUMC Generate Tests") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val file = event.getData(LangDataKeys.VIRTUAL_FILE) ?: return

        val code = String(file.contentsToByteArray())
        val tests = callOmniDevAPI("/api/v1/test/generate", mapOf(
            "code" to code,
            "language" to getLanguageId(project)
        ))

        if (tests != null) {
            WriteCommandAction.runWriteCommandAction(project) {
                val testFile = file.parent.createChildData(project, "${file.nameWithoutExtension}Test.${file.extension}")
                testFile.setBinaryContent(tests.content.toByteArray())
            }
            Messages.showInfoMessage(project, "Test file created", "SBMUMC")
        }
    }
}

class FormalVerificationAction : AnAction("SBMUMC Verify") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val editor = event.getData(LangDataKeys.EDITOR) ?: return
        val code = editor.document.text

        val result = callOmniDevAPI("/api/v1/verify", mapOf(
            "code" to code,
            "claims" to listOf("safety", "termination", "correctness")
        ))

        if (result != null) {
            val status = result.proofs.filter { it.proven }.size
            Messages.showInfoMessage(project, "$status/${result.proofs.size} proofs verified", "Verification")
        }
    }
}

class SystemStatusAction : AnAction("SBMUMC Status") {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val status = callOmniDevAPI("/api/v1/status", emptyMap())

        if (status != null) {
            Messages.showInfoMessage(project,
                "Mode: ${status.mode}\nLatency: ${status.latency_p99_ms}ms\nNodes: ${status.nodes_indexed}",
                "SBMUMC Status"
            )
        }
    }
}

// Utility functions
private fun callOmniDevAPI(endpoint: String, body: Map<String, Any>): Any? {
    return try {
        val client = HttpClients.createDefault()
        val httpPost = HttpPost(URI("http://localhost:8080$endpoint"))
        httpPost.entity = StringEntity(body.toJson())
        httpPost.setHeader("Content-Type", "application/json")
        val response = client.execute(httpPost)
        val result = response.entity.content.bufferedReader().readText()
        parseJson(result)
    } catch (e: Exception) {
        null
    }
}

private fun getLanguageId(project: Project): String {
    return project.basePath?.substringAfterLast('.') ?: "unknown"
}

private fun getGitDiffs(project: Project): List<String> {
    return emptyList() // Git integration
}

private fun openFile(project: Project, filePath: String) {
    val file = com.intellij.openapi.fileEditor.FileDocumentManager.getInstance().getFile(
        com.intellij.openapi.editor.EditorFactory.getInstance().createDocument("")
    )
    // Open file implementation
}

private fun Any.toJson(): String = "{}"
private fun parseJson(text: String): Any? = null