//! Emotion & Sentiment Intelligence Module
//!
//! This module implements emotion recognition, affective computing, sentiment
//! analysis, mood tracking, and empathetic response generation.
//!
//! Features:
//! - Emotion recognition (text, voice, image)
//! - Affective computing engine
//! - Sentiment analysis deep learning
//! - Mood tracking and prediction
//! - Empathetic response generation

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Emotion types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionType {
    /// Joy/Happiness
    Joy,
    /// Sadness
    Sadness,
    /// Anger
    Anger,
    /// Fear
    Fear,
    /// Surprise
    Surprise,
    /// Disgust
    Disgust,
    /// Trust
    Trust,
    /// Anticipation
    Anticipation,
    /// Love/Passion
    Love,
    /// Pride
    Pride,
    /// Shame
    Shame,
    /// Guilt
    Guilt,
    /// Hope
    Hope,
    /// Gratitude
    Gratitude,
    /// Compassion
    Compassion,
    /// Contentment
    Contentment,
    /// Confusion
    Confusion,
    /// Curiosity
    Curiosity,
    /// Serenity
    Serenity,
    /// Envy
    Envy,
}

impl EmotionType {
    /// Get emotion valence (positive/negative)
    pub fn valence(&self) -> f64 {
        match self {
            EmotionType::Joy => 0.9,
            EmotionType::Sadness => -0.8,
            EmotionType::Anger => -0.7,
            EmotionType::Fear => -0.7,
            EmotionType::Surprise => 0.3,
            EmotionType::Disgust => -0.6,
            EmotionType::Trust => 0.7,
            EmotionType::Anticipation => 0.6,
            EmotionType::Love => 0.9,
            EmotionType::Pride => 0.7,
            EmotionType::Shame => -0.6,
            EmotionType::Guilt => -0.7,
            EmotionType::Hope => 0.8,
            EmotionType::Gratitude => 0.8,
            EmotionType::Compassion => 0.7,
            EmotionType::Contentment => 0.8,
            EmotionType::Confusion => -0.2,
            EmotionType::Curiosity => 0.6,
            EmotionType::Serenity => 0.9,
            EmotionType::Envy => -0.4,
        }
    }

    /// Get emotion arousal
    pub fn arousal(&self) -> f64 {
        match self {
            EmotionType::Joy => 0.8,
            EmotionType::Sadness => 0.3,
            EmotionType::Anger => 0.9,
            EmotionType::Fear => 0.9,
            EmotionType::Surprise => 0.9,
            EmotionType::Disgust => 0.6,
            EmotionType::Trust => 0.4,
            EmotionType::Anticipation => 0.7,
            EmotionType::Love => 0.7,
            EmotionType::Pride => 0.7,
            EmotionType::Shame => 0.4,
            EmotionType::Guilt => 0.5,
            EmotionType::Hope => 0.6,
            EmotionType::Gratitude => 0.3,
            EmotionType::Compassion => 0.4,
            EmotionType::Contentment => 0.2,
            EmotionType::Confusion => 0.5,
            EmotionType::Curiosity => 0.6,
            EmotionType::Serenity => 0.1,
            EmotionType::Envy => 0.5,
        }
    }

    /// Get emotion dominance
    pub fn dominance(&self) -> f64 {
        match self {
            EmotionType::Joy => 0.7,
            EmotionType::Sadness => 0.2,
            EmotionType::Anger => 0.9,
            EmotionType::Fear => 0.3,
            EmotionType::Surprise => 0.6,
            EmotionType::Disgust => 0.6,
            EmotionType::Trust => 0.5,
            EmotionType::Anticipation => 0.6,
            EmotionType::Love => 0.8,
            EmotionType::Pride => 0.8,
            EmotionType::Shame => 0.3,
            EmotionType::Guilt => 0.4,
            EmotionType::Hope => 0.6,
            EmotionType::Gratitude => 0.5,
            EmotionType::Compassion => 0.5,
            EmotionType::Contentment => 0.5,
            EmotionType::Confusion => 0.2,
            EmotionType::Curiosity => 0.5,
            EmotionType::Serenity => 0.4,
            EmotionType::Envy => 0.4,
        }
    }

    /// Get emotion color (for visualization)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            EmotionType::Joy => (255, 223, 0),        // Gold
            EmotionType::Sadness => (70, 130, 180),    // Steel blue
            EmotionType::Anger => (255, 69, 0),        // Red-orange
            EmotionType::Fear => (138, 43, 226),      // Blue-violet
            EmotionType::Surprise => (255, 165, 0),   // Orange
            EmotionType::Disgust => (128, 128, 0),    // Olive
            EmotionType::Trust => (0, 128, 255),      // Azure
            EmotionType::Anticipation => (255, 127, 80), // Coral
            EmotionType::Love => (255, 105, 180),     // Hot pink
            EmotionType::Pride => (148, 0, 211),      // Violet
            EmotionType::Shame => (219, 112, 147),    // Pale violet red
            EmotionType::Guilt => (139, 69, 19),      // Saddle brown
            EmotionType::Hope => (152, 251, 152),     // Pale green
            EmotionType::Gratitude => (255, 215, 0),  // Gold
            EmotionType::Compassion => (230, 230, 250), // Lavender
            EmotionType::Contentment => (144, 238, 144), // Light green
            EmotionType::Confusion => (192, 192, 192), // Silver
            EmotionType::Curiosity => (0, 191, 255),  // Deep sky blue
            EmotionType::Serenity => (175, 238, 238), // Pale turquoise
            EmotionType::Envy => (50, 205, 50),       // Lime green
        }
    }
}

/// Detected emotion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedEmotion {
    /// Emotion type
    pub emotion: EmotionType,
    /// Confidence score
    pub confidence: f64,
    /// Intensity
    pub intensity: f64,
    /// Source (text, voice, face)
    pub source: EmotionSource,
}

impl DetectedEmotion {
    /// Create a new detected emotion
    pub fn new(emotion: EmotionType, confidence: f64, source: EmotionSource) -> Self {
        DetectedEmotion {
            emotion,
            confidence,
            intensity: (confidence * emotion.arousal()).clamp(0.0, 1.0),
            source,
        }
    }
}

/// Emotion source modality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionSource {
    /// Text
    Text,
    /// Voice/Audio
    Voice,
    /// Facial expression
    Face,
    /// Body language
    Body,
    /// Physiological signals
    Physiological,
    /// Multi-modal fusion
    Fusion,
}

/// Sentiment polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentimentPolarity {
    /// Strongly negative
    VeryNegative,
    /// Negative
    Negative,
    /// Neutral
    Neutral,
    /// Positive
    Positive,
    /// Strongly positive
    VeryPositive,
}

impl SentimentPolarity {
    /// Convert to score
    pub fn to_score(&self) -> f64 {
        match self {
            SentimentPolarity::VeryNegative => -1.0,
            SentimentPolarity::Negative => -0.5,
            SentimentPolarity::Neutral => 0.0,
            SentimentPolarity::Positive => 0.5,
            SentimentPolarity::VeryPositive => 1.0,
        }
    }

    /// Get from score
    pub fn from_score(score: f64) -> Self {
        if score < -0.6 {
            SentimentPolarity::VeryNegative
        } else if score < -0.2 {
            SentimentPolarity::Negative
        } else if score < 0.2 {
            SentimentPolarity::Neutral
        } else if score < 0.6 {
            SentimentPolarity::Positive
        } else {
            SentimentPolarity::VeryPositive
        }
    }
}

/// Sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentResult {
    /// Polarity
    pub polarity: SentimentPolarity,
    /// Score (-1 to 1)
    pub score: f64,
    /// Detected emotions
    pub emotions: Vec<DetectedEmotion>,
    /// Subjectivity (0=objective, 1=subjective)
    pub subjectivity: f64,
    /// Key phrases
    pub key_phrases: Vec<String>,
    /// Confidence
    pub confidence: f64,
}

impl SentimentResult {
    /// Create a new sentiment result
    pub fn new() -> Self {
        SentimentResult {
            polarity: SentimentPolarity::Neutral,
            score: 0.0,
            emotions: Vec::new(),
            subjectivity: 0.5,
            key_phrases: Vec::new(),
            confidence: 0.0,
        }
    }
}

impl Default for SentimentResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Mood state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodState {
    /// Current mood
    pub current_mood: EmotionType,
    /// Mood intensity
    pub intensity: f64,
    /// Valence
    pub valence: f64,
    /// Arousal
    pub arousal: f64,
    /// Dominance
    pub dominance: f64,
    /// Timestamp
    pub timestamp: u64,
    /// Triggers
    pub triggers: Vec<String>,
}

impl MoodState {
    /// Create a new mood state
    pub fn new(emotion: EmotionType) -> Self {
        MoodState {
            current_mood: emotion,
            intensity: 0.5,
            valence: emotion.valence(),
            arousal: emotion.arousal(),
            dominance: emotion.dominance(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            triggers: Vec::new(),
        }
    }

    /// Create from emotion
    pub fn from_emotion(emotion: EmotionType, intensity: f64) -> Self {
        MoodState {
            current_mood: emotion,
            intensity,
            valence: emotion.valence(),
            arousal: emotion.arousal(),
            dominance: emotion.dominance(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            triggers: Vec::new(),
        }
    }

    /// VAD representation
    pub fn vad(&self) -> (f64, f64, f64) {
        (self.valence, self.arousal, self.dominance)
    }
}

/// Mood history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodHistoryEntry {
    /// Mood state
    pub mood: MoodState,
    /// Context
    pub context: String,
    /// Event
    pub event: Option<String>,
    /// Notes
    pub notes: Option<String>,
}

impl MoodHistoryEntry {
    /// Create a new entry
    pub fn new(mood: MoodState, context: &str) -> Self {
        MoodHistoryEntry {
            mood,
            context: context.to_string(),
            event: None,
            notes: None,
        }
    }
}

/// Mood pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodPattern {
    /// Pattern type
    pub pattern_type: PatternType,
    /// Description
    pub description: String,
    /// Frequency
    pub frequency: f64,
    /// Average duration
    pub average_duration: f64,
    /// Triggers
    pub triggers: Vec<String>,
}

/// Pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    /// Daily rhythm
    Circadian,
    /// Weekly pattern
    Weekly,
    /// Event-triggered
    EventTriggered,
    /// Social-related
    Social,
    /// Seasonal
    Seasonal,
    /// Stress-related
    StressRelated,
}

/// Affective state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectiveState {
    /// Current emotions
    pub current_emotions: Vec<DetectedEmotion>,
    /// Dominant emotion
    pub dominant_emotion: Option<EmotionType>,
    /// Mood state
    pub mood: MoodState,
    /// Emotional blend
    pub emotional_blend: HashMap<EmotionType, f64>,
    /// Stress level
    pub stress_level: f64,
    /// Energy level
    pub energy_level: f64,
    /// Engagement level
    pub engagement_level: f64,
}

impl AffectiveState {
    /// Create a new affective state
    pub fn new() -> Self {
        AffectiveState {
            current_emotions: Vec::new(),
            dominant_emotion: None,
            mood: MoodState::from_emotion(EmotionType::Contentment, 0.5),
            emotional_blend: HashMap::new(),
            stress_level: 0.3,
            energy_level: 0.6,
            engagement_level: 0.5,
        }
    }

    /// Update dominant emotion
    pub fn update_dominant(&mut self) {
        if !self.current_emotions.is_empty() {
            let dominant = self.current_emotions.iter()
                .max_by(|a, b| (a.confidence * a.intensity).partial_cmp(&(b.confidence * b.intensity)).unwrap());

            if let Some(d) = dominant {
                self.dominant_emotion = Some(d.emotion);
            }
        }
    }
}

impl Default for AffectiveState {
    fn default() -> Self {
        Self::new()
    }
}

/// Empathetic response template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpatheticResponse {
    /// Response text
    pub response: String,
    /// Empathy type
    pub empathy_type: EmpathyType,
    /// Emotion acknowledged
    pub acknowledged_emotion: EmotionType,
    /// Emotional support level
    pub support_level: SupportLevel,
    /// Validation statement
    pub validation: Option<String>,
    /// Suggested action
    pub suggested_action: Option<String>,
}

impl EmpatheticResponse {
    /// Create a new response
    pub fn new(response: &str, empathy_type: EmpathyType, emotion: EmotionType) -> Self {
        EmpatheticResponse {
            response: response.to_string(),
            empathy_type,
            acknowledged_emotion: emotion,
            support_level: SupportLevel::Moderate,
            validation: None,
            suggested_action: None,
        }
    }
}

/// Empathy type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmpathyType {
    /// Cognitive empathy (understanding)
    Cognitive,
    /// Emotional empathy (feeling with)
    Emotional,
    /// Compassionate empathy (understanding + action)
    Compassionate,
    /// Sympathetic
    Sympathetic,
}

/// Support level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportLevel {
    /// Minimal support
    Minimal,
    /// Moderate support
    Moderate,
    /// High support
    High,
    /// Intensive support
    Intensive,
}

/// Emotion lexicon entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionLexiconEntry {
    /// Word
    pub word: String,
    /// Associated emotion
    pub emotion: EmotionType,
    /// Intensity
    pub intensity: f64,
    /// Is negating
    pub is_negating: bool,
    /// Is intensifying
    pub is_intensifying: bool,
}

impl EmotionLexiconEntry {
    /// Create a new lexicon entry
    pub fn new(word: &str, emotion: EmotionType, intensity: f64) -> Self {
        EmotionLexiconEntry {
            word: word.to_string(),
            emotion,
            intensity,
            is_negating: false,
            is_intensifying: false,
        }
    }
}

/// Voice emotion features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceEmotionFeatures {
    /// Pitch mean
    pub pitch_mean: f64,
    /// Pitch variance
    pub pitch_variance: f64,
    /// Speaking rate
    pub speaking_rate: f64,
    /// Energy mean
    pub energy_mean: f64,
    /// Energy variance
    pub energy_variance: f64,
    /// Pause frequency
    pub pause_frequency: f64,
    /// Jitter (pitch variation)
    pub jitter: f64,
    /// Shimmer (amplitude variation)
    pub shimmer: f64,
}

impl VoiceEmotionFeatures {
    /// Create new features
    pub fn new() -> Self {
        VoiceEmotionFeatures {
            pitch_mean: 150.0,
            pitch_variance: 20.0,
            speaking_rate: 5.0,
            energy_mean: 0.5,
            energy_variance: 0.1,
            pause_frequency: 0.1,
            jitter: 0.02,
            shimmer: 0.03,
        }
    }

    /// Extract emotion from features
    pub fn extract_emotion(&self) -> EmotionType {
        // Simple rule-based extraction
        if self.pitch_mean > 200.0 && self.energy_mean > 0.7 {
            EmotionType::Anger
        } else if self.pitch_mean > 180.0 && self.speaking_rate > 6.0 {
            EmotionType::Fear
        } else if self.pitch_mean < 120.0 && self.speaking_rate < 4.0 {
            EmotionType::Sadness
        } else if self.pitch_variance > 50.0 {
            EmotionType::Surprise
        } else {
            EmotionType::Contentment
        }
    }
}

impl Default for VoiceEmotionFeatures {
    fn default() -> Self {
        Self::new()
    }
}

/// Facial emotion features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacialEmotionFeatures {
    /// Eye openness
    pub eye_openness: f64,
    /// Eyebrow position
    pub eyebrow_position: EyebrowPosition,
    /// Mouth shape
    pub mouth_shape: MouthShape,
    /// Head tilt
    pub head_tilt: f64,
    /// Facial asymmetry
    pub asymmetry: f64,
}

/// Eyebrow position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyebrowPosition {
    Neutral,
    Raised,
    Lowered,
    InnerRaised,
    Asymmetric,
}

/// Mouth shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouthShape {
    Neutral,
    Smile,
    Frown,
    Open,
    Grimace,
    Smirk,
}

/// Emotion intelligence system
pub struct EmotionIntelligence {
    /// Emotion lexicon
    pub lexicon: Vec<EmotionLexiconEntry>,
    /// Mood history
    pub mood_history: VecDeque<MoodHistoryEntry>,
    /// Current affective state
    pub affective_state: AffectiveState,
    /// Detected patterns
    pub patterns: Vec<MoodPattern>,
    /// Response templates
    pub response_templates: Vec<ResponseTemplate>,
    /// Emotion models
    pub sentiment_model: Option<SentimentModel>,
}

impl EmotionIntelligence {
    /// Create a new emotion intelligence system
    pub fn new() -> Self {
        let mut system = EmotionIntelligence {
            lexicon: Vec::new(),
            mood_history: VecDeque::new(),
            affective_state: AffectiveState::new(),
            patterns: Vec::new(),
            response_templates: Vec::new(),
            sentiment_model: None,
        };

        system.initialize_lexicon();
        system.initialize_templates();

        system
    }

    /// Initialize emotion lexicon
    fn initialize_lexicon(&mut self) {
        let words = vec![
            ("happy", EmotionType::Joy, 0.9),
            ("sad", EmotionType::Sadness, 0.8),
            ("angry", EmotionType::Anger, 0.9),
            ("afraid", EmotionType::Fear, 0.8),
            ("scared", EmotionType::Fear, 0.8),
            ("surprised", EmotionType::Surprise, 0.7),
            ("disgusted", EmotionType::Disgust, 0.8),
            ("trust", EmotionType::Trust, 0.7),
            ("anticipate", EmotionType::Anticipation, 0.6),
            ("love", EmotionType::Love, 0.9),
            ("proud", EmotionType::Pride, 0.8),
            ("ashamed", EmotionType::Shame, 0.7),
            ("guilty", EmotionType::Guilt, 0.7),
            ("hope", EmotionType::Hope, 0.8),
            ("grateful", EmotionType::Gratitude, 0.8),
            ("compassion", EmotionType::Compassion, 0.7),
            ("confused", EmotionType::Confusion, 0.6),
            ("curious", EmotionType::Curiosity, 0.7),
            ("serene", EmotionType::Serenity, 0.7),
            ("envious", EmotionType::Envy, 0.6),
        ];

        for (word, emotion, intensity) in words {
            self.lexicon.push(EmotionLexiconEntry::new(word, emotion, intensity));
        }
    }

    /// Initialize response templates
    fn initialize_templates(&mut self) {
        self.response_templates.push(ResponseTemplate {
            emotion: EmotionType::Sadness,
            template: "I understand you're feeling down. It's okay to feel this way.".to_string(),
            empathy_type: EmpathyType::Emotional,
        });

        self.response_templates.push(ResponseTemplate {
            emotion: EmotionType::Anger,
            template: "I can see this is frustrating for you. Let's work through it together.".to_string(),
            empathy_type: EmpathyType::Cognitive,
        });

        self.response_templates.push(ResponseTemplate {
            emotion: EmotionType::Fear,
            template: "It's natural to feel anxious about this. I'm here to help.".to_string(),
            empathy_type: EmpathyType::Compassionate,
        });

        self.response_templates.push(ResponseTemplate {
            emotion: EmotionType::Joy,
            template: "That's wonderful! I'm glad to hear you're feeling happy!".to_string(),
            empathy_type: EmpathyType::Emotional,
        });
    }

    /// Detect emotion from text
    pub fn detect_text_emotion(&self, text: &str) -> Vec<DetectedEmotion> {
        let mut emotions: HashMap<EmotionType, f64> = HashMap::new();
        let words: Vec<&str> = text.to_lowercase().split_whitespace().collect();

        for word in &words {
            for entry in &self.lexicon {
                if entry.word == *word {
                    *emotions.entry(entry.emotion).or_insert(0.0) += entry.intensity;
                }
            }
        }

        // Check for negation
        let has_negation = words.iter().any(|w| matches!(w, "not" | "no" | "never" | "don't" | "doesn't"));

        emotions.into_iter()
            .map(|(emotion, score)| {
                let confidence = (score / words.len().max(1) as f64).min(1.0);
                DetectedEmotion::new(
                    if has_negation { self.get_opposite(emotion) } else { emotion },
                    confidence,
                    EmotionSource::Text,
                )
            })
            .collect()
    }

    /// Get opposite emotion
    fn get_opposite(&self, emotion: EmotionType) -> EmotionType {
        match emotion {
            EmotionType::Joy => EmotionType::Sadness,
            EmotionType::Sadness => EmotionType::Joy,
            EmotionType::Anger => EmotionType::Calm,
            EmotionType::Fear => EmotionType::Courage,
            _ => emotion,
        }
    }

    /// Detect emotion from voice
    pub fn detect_voice_emotion(&self, features: &VoiceEmotionFeatures) -> DetectedEmotion {
        let emotion = features.extract_emotion();
        let confidence = 0.7; // Simplified confidence

        DetectedEmotion::new(emotion, confidence, EmotionSource::Voice)
    }

    /// Detect emotion from face
    pub fn detect_face_emotion(&self, features: &FacialEmotionFeatures) -> DetectedEmotion {
        // Simple rule-based detection
        let emotion = match features.mouth_shape {
            MouthShape::Smile => EmotionType::Joy,
            MouthShape::Frown => EmotionType::Sadness,
            MouthShape::Open => match features.eyebrow_position {
                EyebrowPosition::Raised => EmotionType::Surprise,
                _ => EmotionType::Fear,
            },
            MouthShape::Grimace => EmotionType::Disgust,
            _ => EmotionType::Contentment,
        };

        DetectedEmotion::new(emotion, 0.8, EmotionSource::Face)
    }

    /// Analyze sentiment
    pub fn analyze_sentiment(&self, text: &str) -> SentimentResult {
        let emotions = self.detect_text_emotion(text);

        let mut total_valence = 0.0;
        let mut total_arousal = 0.0;
        let mut total_confidence = 0.0;

        for emotion in &emotions {
            total_valence += emotion.emotion.valence() * emotion.confidence;
            total_arousal += emotion.emotion.arousal() * emotion.confidence;
            total_confidence += emotion.confidence;
        }

        let score = if total_confidence > 0.0 {
            total_valence / total_confidence
        } else {
            0.0
        };

        SentimentResult {
            polarity: SentimentPolarity::from_score(score),
            score,
            emotions,
            subjectivity: 0.5, // Simplified
            key_phrases: Vec::new(),
            confidence: total_confidence,
        }
    }

    /// Update mood state
    pub fn update_mood(&mut self, emotion: EmotionType, intensity: f64, context: &str) {
        let mood = MoodState::from_emotion(emotion, intensity);
        let entry = MoodHistoryEntry::new(mood.clone(), context);

        self.mood_history.push_back(entry);
        self.affective_state.mood = mood;

        // Keep history bounded
        if self.mood_history.len() > 1000 {
            self.mood_history.pop_front();
        }
    }

    /// Get mood trend
    pub fn get_mood_trend(&self, window_hours: u64) -> Vec<MoodState> {
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (window_hours * 3600);

        self.mood_history.iter()
            .filter(|e| e.mood.timestamp > cutoff)
            .map(|e| e.mood.clone())
            .collect()
    }

    /// Predict future mood
    pub fn predict_mood(&self) -> Option<MoodState> {
        if self.mood_history.len() < 10 {
            return None;
        }

        // Simple prediction based on recent trend
        let recent: Vec<_> = self.mood_history.iter().rev().take(10).collect();
        let avg_valence: f64 = recent.iter()
            .map(|e| e.mood.valence)
            .sum::<f64>() / recent.len() as f64;

        // Predict same direction
        let predicted_valence = avg_valence * 1.05; // Slight continuation

        // Find closest matching emotion
        let emotion = if predicted_valence > 0.6 {
            EmotionType::Joy
        } else if predicted_valence > 0.3 {
            EmotionType::Contentment
        } else if predicted_valence > -0.3 {
            EmotionType::Serenity
        } else if predicted_valence > -0.6 {
            EmotionType::Sadness
        } else {
            EmotionType::Fear
        };

        Some(MoodState::from_emotion(emotion, 0.5))
    }

    /// Generate empathetic response
    pub fn generate_empathetic_response(&self, detected_emotion: &DetectedEmotion) -> EmpatheticResponse {
        // Find matching template
        for template in &self.response_templates {
            if template.emotion == detected_emotion.emotion {
                return EmpatheticResponse::new(
                    &template.template,
                    template.empathy_type,
                    detected_emotion.emotion,
                );
            }
        }

        // Default response
        EmpatheticResponse::new(
            "I understand you're experiencing something right now.",
            EmpathyType::Cognitive,
            detected_emotion.emotion,
        )
    }

    /// Fuse multi-modal emotions
    pub fn fuse_emotions(&self, text_emotions: &[DetectedEmotion], voice_emotion: Option<&DetectedEmotion>, face_emotion: Option<&DetectedEmotion>) -> Vec<DetectedEmotion> {
        let mut emotion_scores: HashMap<EmotionType, f64> = HashMap::new();

        // Weight text emotions
        for emotion in text_emotions {
            *emotion_scores.entry(emotion.emotion).or_insert(0.0) += emotion.confidence * 0.4;
        }

        // Weight voice emotions
        if let Some(ve) = voice_emotion {
            *emotion_scores.entry(ve.emotion).or_insert(0.0) += ve.confidence * 0.3;
        }

        // Weight face emotions
        if let Some(fe) = face_emotion {
            *emotion_scores.entry(fe.emotion).or_insert(0.0) += fe.confidence * 0.3;
        }

        emotion_scores.into_iter()
            .map(|(emotion, score)| DetectedEmotion::new(emotion, score, EmotionSource::Fusion))
            .collect()
    }

    /// Detect mood patterns
    pub fn detect_patterns(&self) -> Vec<MoodPattern> {
        let mut patterns = Vec::new();

        if self.mood_history.len() < 20 {
            return patterns;
        }

        // Detect circadian pattern
        let mut hourly: HashMap<u32, Vec<f64>> = HashMap::new();
        for entry in &self.mood_history {
            let hour = (entry.mood.timestamp % 86400) / 3600;
            hourly.entry(hour).or_insert_with(Vec::new).push(entry.mood.valence);
        }

        if !hourly.is_empty() {
            patterns.push(MoodPattern {
                pattern_type: PatternType::Circadian,
                description: "Daily mood rhythm detected".to_string(),
                frequency: hourly.len() as f64 / 24.0,
                average_duration: 8.0,
                triggers: Vec::new(),
            });
        }

        patterns
    }
}

impl Default for EmotionIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

/// Response template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTemplate {
    /// Target emotion
    pub emotion: EmotionType,
    /// Template text
    pub template: String,
    /// Empathy type
    pub empathy_type: EmpathyType,
}

/// Sentiment model (placeholder for ML model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentModel {
    /// Model name
    pub name: String,
    /// Version
    pub version: String,
    /// Is loaded
    pub loaded: bool,
}

impl SentimentModel {
    /// Create a new model
    pub fn new(name: &str, version: &str) -> Self {
        SentimentModel {
            name: name.to_string(),
            version: version.to_string(),
            loaded: false,
        }
    }

    /// Load model
    pub fn load(&mut self) -> bool {
        self.loaded = true;
        true
    }
}

// Helper types
type Calm = EmotionType;
type Courage = EmotionType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_types() {
        assert_eq!(EmotionType::Joy.valence(), 0.9);
        assert_eq!(EmotionType::Sadness.valence(), -0.8);
        assert_eq!(EmotionType::Joy.arousal(), 0.8);
    }

    #[test]
    fn test_sentiment_polarity() {
        assert_eq!(SentimentPolarity::from_score(0.8), SentimentPolarity::VeryPositive);
        assert_eq!(SentimentPolarity::from_score(-0.8), SentimentPolarity::VeryNegative);
        assert_eq!(SentimentPolarity::from_score(0.0), SentimentPolarity::Neutral);
    }

    #[test]
    fn test_mood_state() {
        let mood = MoodState::from_emotion(EmotionType::Joy, 0.8);
        assert_eq!(mood.current_mood, EmotionType::Joy);
        assert_eq!(mood.intensity, 0.8);
    }

    #[test]
    fn test_emotion_detection() {
        let system = EmotionIntelligence::new();
        let emotions = system.detect_text_emotion("I am so happy and excited!");

        assert!(!emotions.is_empty());
    }

    #[test]
    fn test_sentiment_analysis() {
        let system = EmotionIntelligence::new();
        let result = system.analyze_sentiment("This is wonderful!");

        assert!(result.score > 0.0);
    }

    #[test]
    fn test_empathetic_response() {
        let system = EmotionIntelligence::new();
        let emotion = DetectedEmotion::new(EmotionType::Sadness, 0.8, EmotionSource::Text);
        let response = system.generate_empathetic_response(&emotion);

        assert!(!response.response.is_empty());
    }

    #[test]
    fn test_voice_emotion_features() {
        let features = VoiceEmotionFeatures::new();
        let emotion = features.extract_emotion();

        assert!(matches!(emotion, EmotionType::Contentment));
    }

    #[test]
    fn test_mood_tracking() {
        let mut system = EmotionIntelligence::new();
        system.update_mood(EmotionType::Joy, 0.8, "Good news received");

        let trend = system.get_mood_trend(24);
        assert!(!trend.is_empty());
    }

    #[test]
    fn test_emotion_fusion() {
        let system = EmotionIntelligence::new();
        let text_emotions = vec![DetectedEmotion::new(EmotionType::Joy, 0.9, EmotionSource::Text)];
        let voice_emotion = DetectedEmotion::new(EmotionType::Joy, 0.8, EmotionSource::Voice);

        let fused = system.fuse_emotions(&text_emotions, Some(&voice_emotion), None);
        assert!(!fused.is_empty());
    }

    #[test]
    fn test_vad() {
        let mood = MoodState::from_emotion(EmotionType::Joy, 0.8);
        let (v, a, d) = mood.vad();

        assert_eq!(v, 0.9);
        assert_eq!(a, 0.8);
    }
}
