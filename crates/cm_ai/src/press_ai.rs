//! Press AI - Press conference question and response generation.

/// Context for press questions.
#[derive(Debug, Clone)]
pub struct PressContext {
    pub event_type: PressEventType,
    pub result: Option<MatchResult>,
    pub league_position: u8,
    pub form: Form,
    pub upcoming_opponent: Option<String>,
    pub recent_transfers: Vec<TransferNews>,
    pub injury_news: Vec<String>,
}

impl Default for PressContext {
    fn default() -> Self {
        Self {
            event_type: PressEventType::PreMatch,
            result: None,
            league_position: 10,
            form: Form::Average,
            upcoming_opponent: None,
            recent_transfers: Vec::new(),
            injury_news: Vec::new(),
        }
    }
}

/// Type of press event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressEventType {
    PreMatch,
    PostMatch,
    Weekly,
    TransferWindow,
    Injury,
}

/// Match result for post-match press.
#[derive(Debug, Clone, Copy)]
pub struct MatchResult {
    pub own_score: u8,
    pub opponent_score: u8,
    pub is_home: bool,
}

impl MatchResult {
    pub fn is_win(&self) -> bool {
        self.own_score > self.opponent_score
    }
    
    pub fn is_draw(&self) -> bool {
        self.own_score == self.opponent_score
    }
    
    pub fn is_loss(&self) -> bool {
        self.own_score < self.opponent_score
    }
}

/// Team form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    Excellent,
    Good,
    Average,
    Poor,
    Terrible,
}

/// Transfer news for press.
#[derive(Debug, Clone)]
pub struct TransferNews {
    pub player_name: String,
    pub is_incoming: bool,
}

/// Press question types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    MatchPreview,
    MatchReview,
    FormQuestion,
    TacticsQuestion,
    TransferQuestion,
    InjuryQuestion,
    LeaguePosition,
    OpponentQuestion,
    PlayerPerformance,
    FutureProspects,
}

/// A generated press question.
#[derive(Debug, Clone)]
pub struct PressQuestion {
    pub question_type: QuestionType,
    pub text: String,
    pub tone: Tone,
}

/// Question tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Friendly,
    Neutral,
    Challenging,
    Provocative,
}

/// Generate press questions based on context.
pub fn generate_questions(context: &PressContext, count: usize) -> Vec<PressQuestion> {
    let mut questions = Vec::new();
    
    match context.event_type {
        PressEventType::PreMatch => {
            questions.push(generate_match_preview_question(context));
            
            if let Some(opponent) = &context.upcoming_opponent {
                questions.push(PressQuestion {
                    question_type: QuestionType::OpponentQuestion,
                    text: format!("What do you make of {} as opponents?", opponent),
                    tone: Tone::Neutral,
                });
            }
            
            questions.push(generate_tactics_question(context));
        }
        PressEventType::PostMatch => {
            if let Some(result) = &context.result {
                questions.push(generate_match_review_question(result));
                questions.push(generate_performance_question(result));
            }
        }
        PressEventType::Weekly => {
            questions.push(generate_form_question(context));
            questions.push(generate_league_position_question(context));
        }
        PressEventType::TransferWindow => {
            questions.push(generate_transfer_question(context));
        }
        PressEventType::Injury => {
            if !context.injury_news.is_empty() {
                questions.push(generate_injury_question(context));
            }
        }
    }
    
    // Always add a general question if we don't have enough
    while questions.len() < count {
        questions.push(PressQuestion {
            question_type: QuestionType::FutureProspects,
            text: "What are your hopes for the rest of the season?".into(),
            tone: Tone::Neutral,
        });
    }
    
    questions.truncate(count);
    questions
}

fn generate_match_preview_question(context: &PressContext) -> PressQuestion {
    let text = match &context.upcoming_opponent {
        Some(opponent) => format!("How are you preparing for the match against {}?", opponent),
        None => "How is preparation going for your next match?".into(),
    };
    
    PressQuestion {
        question_type: QuestionType::MatchPreview,
        text,
        tone: Tone::Neutral,
    }
}

fn generate_match_review_question(result: &MatchResult) -> PressQuestion {
    let text = if result.is_win() {
        "Congratulations on the win. What were the key factors?".into()
    } else if result.is_draw() {
        "How do you assess the draw today?".into()
    } else {
        "It was a disappointing result. What went wrong?".into()
    };
    
    let tone = if result.is_loss() { Tone::Challenging } else { Tone::Friendly };
    
    PressQuestion {
        question_type: QuestionType::MatchReview,
        text,
        tone,
    }
}

fn generate_performance_question(result: &MatchResult) -> PressQuestion {
    let text = if result.own_score >= 3 {
        "The attack looked sharp today. Are you pleased with the goalscoring?".into()
    } else if result.opponent_score >= 3 {
        "Defensively it was a tough day. What needs to improve?".into()
    } else {
        "How would you rate the overall performance?".into()
    };
    
    PressQuestion {
        question_type: QuestionType::PlayerPerformance,
        text,
        tone: Tone::Neutral,
    }
}

fn generate_form_question(context: &PressContext) -> PressQuestion {
    let (text, tone) = match context.form {
        Form::Excellent => (
            "The team is in excellent form. What's the secret?".into(),
            Tone::Friendly,
        ),
        Form::Good => (
            "Results have been positive recently. Can you maintain this form?".into(),
            Tone::Neutral,
        ),
        Form::Average => (
            "Results have been mixed recently. Are you concerned?".into(),
            Tone::Neutral,
        ),
        Form::Poor => (
            "Results have been disappointing. What's going wrong?".into(),
            Tone::Challenging,
        ),
        Form::Terrible => (
            "The team is in crisis. How do you turn this around?".into(),
            Tone::Provocative,
        ),
    };
    
    PressQuestion {
        question_type: QuestionType::FormQuestion,
        text,
        tone,
    }
}

fn generate_tactics_question(_context: &PressContext) -> PressQuestion {
    PressQuestion {
        question_type: QuestionType::TacticsQuestion,
        text: "Can you tell us about your tactical approach?".into(),
        tone: Tone::Neutral,
    }
}

fn generate_league_position_question(context: &PressContext) -> PressQuestion {
    let (text, tone) = if context.league_position <= 3 {
        (
            "You're near the top of the table. Are you thinking about the title?".into(),
            Tone::Friendly,
        )
    } else if context.league_position <= 6 {
        (
            "European places are within reach. Is that the target?".into(),
            Tone::Neutral,
        )
    } else if context.league_position >= 18 {
        (
            "You're in the relegation zone. How worried are you?".into(),
            Tone::Provocative,
        )
    } else {
        (
            format!("You're currently {}th in the table. Where do you see the team finishing?", context.league_position),
            Tone::Neutral,
        )
    };
    
    PressQuestion {
        question_type: QuestionType::LeaguePosition,
        text,
        tone,
    }
}

fn generate_transfer_question(context: &PressContext) -> PressQuestion {
    if let Some(transfer) = context.recent_transfers.first() {
        let text = if transfer.is_incoming {
            format!("Can you tell us about the signing of {}?", transfer.player_name)
        } else {
            format!("Why did you decide to let {} go?", transfer.player_name)
        };
        
        PressQuestion {
            question_type: QuestionType::TransferQuestion,
            text,
            tone: Tone::Neutral,
        }
    } else {
        PressQuestion {
            question_type: QuestionType::TransferQuestion,
            text: "Are you looking to make any signings in this window?".into(),
            tone: Tone::Neutral,
        }
    }
}

fn generate_injury_question(context: &PressContext) -> PressQuestion {
    let text = if let Some(player) = context.injury_news.first() {
        format!("What's the latest on {}'s injury?", player)
    } else {
        "How is the squad's fitness looking?".into()
    };
    
    PressQuestion {
        question_type: QuestionType::InjuryQuestion,
        text,
        tone: Tone::Neutral,
    }
}

/// Response tone for answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTone {
    Confident,
    Humble,
    Diplomatic,
    Defiant,
    Deflecting,
}

/// A press response.
#[derive(Debug, Clone)]
pub struct PressResponse {
    pub text: String,
    pub tone: ResponseTone,
    pub morale_effect: i8,  // Effect on squad morale (-10 to +10)
    pub media_reaction: MediaReaction,
}

/// Media reaction to response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaReaction {
    Positive,
    Neutral,
    Negative,
    Controversial,
}

/// Generate AI response to a press question.
pub fn generate_response(question: &PressQuestion, context: &PressContext) -> PressResponse {
    match question.question_type {
        QuestionType::MatchPreview => generate_preview_response(context),
        QuestionType::MatchReview => generate_review_response(context),
        QuestionType::FormQuestion => generate_form_response(context),
        QuestionType::TacticsQuestion => generate_tactics_response(),
        QuestionType::TransferQuestion => generate_transfer_response(),
        QuestionType::InjuryQuestion => generate_injury_response(context),
        QuestionType::LeaguePosition => generate_position_response(context),
        QuestionType::OpponentQuestion => generate_opponent_response(),
        QuestionType::PlayerPerformance => generate_performance_response(context),
        QuestionType::FutureProspects => generate_prospects_response(context),
    }
}

fn generate_preview_response(context: &PressContext) -> PressResponse {
    let (text, tone) = match context.form {
        Form::Excellent | Form::Good => (
            "We're confident. The squad is in good shape and we've prepared well.".into(),
            ResponseTone::Confident,
        ),
        Form::Average => (
            "We're taking nothing for granted. Every match is a challenge and we'll give our best.".into(),
            ResponseTone::Diplomatic,
        ),
        _ => (
            "We know we need a result. The players are determined to turn things around.".into(),
            ResponseTone::Defiant,
        ),
    };
    
    PressResponse {
        text,
        tone,
        morale_effect: 1,
        media_reaction: MediaReaction::Neutral,
    }
}

fn generate_review_response(context: &PressContext) -> PressResponse {
    let result = context.result.as_ref();
    
    let (text, tone, morale) = match result {
        Some(r) if r.is_win() => (
            "A great performance from the team. We executed our plan and got the result we deserved.".into(),
            ResponseTone::Confident,
            3,
        ),
        Some(r) if r.is_draw() => (
            "We can be satisfied with a point. There were positives to take from the game.".into(),
            ResponseTone::Diplomatic,
            0,
        ),
        Some(r) if r.is_loss() => (
            "Disappointing result but we have to learn from it and move on to the next game.".into(),
            ResponseTone::Humble,
            -2,
        ),
        _ => (
            "We'll review the performance and prepare for what's next.".into(),
            ResponseTone::Diplomatic,
            0,
        ),
    };
    
    PressResponse {
        text,
        tone,
        morale_effect: morale,
        media_reaction: MediaReaction::Neutral,
    }
}

fn generate_form_response(context: &PressContext) -> PressResponse {
    match context.form {
        Form::Excellent => PressResponse {
            text: "The team is playing with confidence. Long may it continue.".into(),
            tone: ResponseTone::Confident,
            morale_effect: 2,
            media_reaction: MediaReaction::Positive,
        },
        Form::Good => PressResponse {
            text: "We've been working hard and it's showing in results.".into(),
            tone: ResponseTone::Humble,
            morale_effect: 1,
            media_reaction: MediaReaction::Positive,
        },
        Form::Average => PressResponse {
            text: "There's room for improvement but we're heading in the right direction.".into(),
            tone: ResponseTone::Diplomatic,
            morale_effect: 0,
            media_reaction: MediaReaction::Neutral,
        },
        Form::Poor => PressResponse {
            text: "We're working hard to turn things around. The players are giving everything.".into(),
            tone: ResponseTone::Defiant,
            morale_effect: 1,
            media_reaction: MediaReaction::Neutral,
        },
        Form::Terrible => PressResponse {
            text: "It's been a difficult period but I believe in this squad. We'll come through it.".into(),
            tone: ResponseTone::Defiant,
            morale_effect: 2,
            media_reaction: MediaReaction::Neutral,
        },
    }
}

fn generate_tactics_response() -> PressResponse {
    PressResponse {
        text: "I don't want to give too much away, but we have a clear plan for the game.".into(),
        tone: ResponseTone::Deflecting,
        morale_effect: 0,
        media_reaction: MediaReaction::Neutral,
    }
}

fn generate_transfer_response() -> PressResponse {
    PressResponse {
        text: "We're always looking to improve the squad, but I won't comment on speculation.".into(),
        tone: ResponseTone::Diplomatic,
        morale_effect: 0,
        media_reaction: MediaReaction::Neutral,
    }
}

fn generate_injury_response(context: &PressContext) -> PressResponse {
    if context.injury_news.is_empty() {
        PressResponse {
            text: "The squad is fit and we have good options available.".into(),
            tone: ResponseTone::Confident,
            morale_effect: 1,
            media_reaction: MediaReaction::Positive,
        }
    } else {
        PressResponse {
            text: "We're monitoring the situation. The medical team is working hard on recovery.".into(),
            tone: ResponseTone::Diplomatic,
            morale_effect: 0,
            media_reaction: MediaReaction::Neutral,
        }
    }
}

fn generate_position_response(context: &PressContext) -> PressResponse {
    if context.league_position <= 3 {
        PressResponse {
            text: "We're focused on each game as it comes. The table doesn't lie but there's a long way to go.".into(),
            tone: ResponseTone::Humble,
            morale_effect: 1,
            media_reaction: MediaReaction::Positive,
        }
    } else if context.league_position >= 18 {
        PressResponse {
            text: "We know the situation. The players are fighting and we believe we can turn it around.".into(),
            tone: ResponseTone::Defiant,
            morale_effect: 2,
            media_reaction: MediaReaction::Neutral,
        }
    } else {
        PressResponse {
            text: "We're taking it one game at a time. The aim is always to finish as high as possible.".into(),
            tone: ResponseTone::Diplomatic,
            morale_effect: 0,
            media_reaction: MediaReaction::Neutral,
        }
    }
}

fn generate_opponent_response() -> PressResponse {
    PressResponse {
        text: "We respect every opponent. They have quality players but so do we.".into(),
        tone: ResponseTone::Diplomatic,
        morale_effect: 0,
        media_reaction: MediaReaction::Neutral,
    }
}

fn generate_performance_response(context: &PressContext) -> PressResponse {
    if let Some(result) = &context.result {
        if result.own_score >= 3 {
            PressResponse {
                text: "The attackers were clinical today. That's what we work on in training.".into(),
                tone: ResponseTone::Confident,
                morale_effect: 2,
                media_reaction: MediaReaction::Positive,
            }
        } else if result.opponent_score >= 3 {
            PressResponse {
                text: "Defensively we need to be better. We'll work on that this week.".into(),
                tone: ResponseTone::Humble,
                morale_effect: -1,
                media_reaction: MediaReaction::Neutral,
            }
        } else {
            PressResponse {
                text: "Overall a solid performance. Some areas to improve but plenty of positives.".into(),
                tone: ResponseTone::Diplomatic,
                morale_effect: 0,
                media_reaction: MediaReaction::Neutral,
            }
        }
    } else {
        PressResponse {
            text: "The team is performing well. We're always looking to improve.".into(),
            tone: ResponseTone::Diplomatic,
            morale_effect: 0,
            media_reaction: MediaReaction::Neutral,
        }
    }
}

fn generate_prospects_response(context: &PressContext) -> PressResponse {
    match context.form {
        Form::Excellent | Form::Good => PressResponse {
            text: "We're optimistic about what we can achieve. The squad has great potential.".into(),
            tone: ResponseTone::Confident,
            morale_effect: 2,
            media_reaction: MediaReaction::Positive,
        },
        _ => PressResponse {
            text: "We take it game by game. The most important match is always the next one.".into(),
            tone: ResponseTone::Diplomatic,
            morale_effect: 0,
            media_reaction: MediaReaction::Neutral,
        },
    }
}

/// Legacy function for compatibility.
pub fn generate_question(context: &str) -> String {
    let ctx = PressContext::default();
    let questions = generate_questions(&ctx, 1);
    questions.first()
        .map(|q| q.text.clone())
        .unwrap_or_else(|| "How do you feel about the team's performance?".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_prematch_questions() {
        let context = PressContext {
            event_type: PressEventType::PreMatch,
            upcoming_opponent: Some("Manchester United".into()),
            ..Default::default()
        };
        
        let questions = generate_questions(&context, 3);
        
        assert_eq!(questions.len(), 3);
        assert!(questions.iter().any(|q| q.question_type == QuestionType::MatchPreview));
    }

    #[test]
    fn test_generate_postmatch_questions_win() {
        let context = PressContext {
            event_type: PressEventType::PostMatch,
            result: Some(MatchResult {
                own_score: 3,
                opponent_score: 1,
                is_home: true,
            }),
            ..Default::default()
        };
        
        let questions = generate_questions(&context, 2);
        
        assert!(!questions.is_empty());
        // Winning should have friendly tone
        assert!(questions.iter().any(|q| q.tone == Tone::Friendly || q.tone == Tone::Neutral));
    }

    #[test]
    fn test_generate_postmatch_questions_loss() {
        let context = PressContext {
            event_type: PressEventType::PostMatch,
            result: Some(MatchResult {
                own_score: 0,
                opponent_score: 3,
                is_home: true,
            }),
            ..Default::default()
        };
        
        let questions = generate_questions(&context, 2);
        
        assert!(!questions.is_empty());
        // Losing should have challenging tone
        assert!(questions.iter().any(|q| q.tone == Tone::Challenging || q.tone == Tone::Neutral));
    }

    #[test]
    fn test_form_question_tone() {
        let good_context = PressContext {
            event_type: PressEventType::Weekly,
            form: Form::Excellent,
            ..Default::default()
        };
        
        let bad_context = PressContext {
            event_type: PressEventType::Weekly,
            form: Form::Terrible,
            ..Default::default()
        };
        
        let good_question = generate_form_question(&good_context);
        let bad_question = generate_form_question(&bad_context);
        
        assert_eq!(good_question.tone, Tone::Friendly);
        assert_eq!(bad_question.tone, Tone::Provocative);
    }

    #[test]
    fn test_generate_response_confident_after_win() {
        let context = PressContext {
            event_type: PressEventType::PostMatch,
            result: Some(MatchResult {
                own_score: 2,
                opponent_score: 0,
                is_home: true,
            }),
            ..Default::default()
        };
        
        let question = PressQuestion {
            question_type: QuestionType::MatchReview,
            text: "Great result!".into(),
            tone: Tone::Friendly,
        };
        
        let response = generate_response(&question, &context);
        
        assert_eq!(response.tone, ResponseTone::Confident);
        assert!(response.morale_effect > 0);
    }

    #[test]
    fn test_generate_response_humble_after_loss() {
        let context = PressContext {
            event_type: PressEventType::PostMatch,
            result: Some(MatchResult {
                own_score: 0,
                opponent_score: 2,
                is_home: true,
            }),
            ..Default::default()
        };
        
        let question = PressQuestion {
            question_type: QuestionType::MatchReview,
            text: "Disappointing?".into(),
            tone: Tone::Challenging,
        };
        
        let response = generate_response(&question, &context);
        
        assert_eq!(response.tone, ResponseTone::Humble);
    }

    #[test]
    fn test_match_result_helpers() {
        let win = MatchResult { own_score: 2, opponent_score: 1, is_home: true };
        let draw = MatchResult { own_score: 1, opponent_score: 1, is_home: true };
        let loss = MatchResult { own_score: 0, opponent_score: 2, is_home: false };
        
        assert!(win.is_win());
        assert!(!win.is_draw());
        assert!(!win.is_loss());
        
        assert!(!draw.is_win());
        assert!(draw.is_draw());
        assert!(!draw.is_loss());
        
        assert!(!loss.is_win());
        assert!(!loss.is_draw());
        assert!(loss.is_loss());
    }

    #[test]
    fn test_league_position_questions() {
        // Top of table
        let top_context = PressContext {
            event_type: PressEventType::Weekly,
            league_position: 1,
            ..Default::default()
        };
        
        let top_q = generate_league_position_question(&top_context);
        assert_eq!(top_q.tone, Tone::Friendly);
        
        // Relegation zone
        let bottom_context = PressContext {
            event_type: PressEventType::Weekly,
            league_position: 20,
            ..Default::default()
        };
        
        let bottom_q = generate_league_position_question(&bottom_context);
        assert_eq!(bottom_q.tone, Tone::Provocative);
    }

    #[test]
    fn test_transfer_question_with_news() {
        let context = PressContext {
            event_type: PressEventType::TransferWindow,
            recent_transfers: vec![
                TransferNews {
                    player_name: "John Smith".into(),
                    is_incoming: true,
                }
            ],
            ..Default::default()
        };
        
        let question = generate_transfer_question(&context);
        
        assert!(question.text.contains("John Smith"));
        assert_eq!(question.question_type, QuestionType::TransferQuestion);
    }

    #[test]
    fn test_defiant_response_in_bad_form() {
        let context = PressContext {
            form: Form::Terrible,
            ..Default::default()
        };
        
        let response = generate_form_response(&context);
        
        assert_eq!(response.tone, ResponseTone::Defiant);
        assert!(response.morale_effect > 0); // Defiant boosts morale
    }

    #[test]
    fn test_legacy_generate_question() {
        let question = generate_question("test");
        assert!(!question.is_empty());
    }
}
