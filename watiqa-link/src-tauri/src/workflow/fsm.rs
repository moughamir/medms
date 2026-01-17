use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentState {
    Registered, // Enregistré au Bureau d'Ordre
    Dispatched, // Transmis au service concerné
    Annotated,  // Annoté par le chef de service
    Signed,     // Signé par l'autorité compétente
    Archived,   // Archivé
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: DocumentState,
    pub to: DocumentState,
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub notes: Option<String>,
}

pub struct WorkflowEngine {
    current_state: DocumentState,
    history: Vec<StateTransition>,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self {
            current_state: DocumentState::Registered,
            history: Vec::new(),
        }
    }

    pub fn transition(
        &mut self,
        to: DocumentState,
        agent_id: String,
        notes: Option<String>,
    ) -> Result<(), String> {
        let valid = match (self.current_state, to) {
            (DocumentState::Registered, DocumentState::Dispatched) => true,
            (DocumentState::Dispatched, DocumentState::Annotated) => true,
            (DocumentState::Annotated, DocumentState::Signed) => true,
            (DocumentState::Signed, DocumentState::Archived) => true,
            _ => false,
        };

        if !valid {
            return Err(format!(
                "Invalid transition from {:?} to {:?}",
                self.current_state, to
            ));
        }

        self.history.push(StateTransition {
            from: self.current_state,
            to,
            timestamp: Utc::now(),
            agent_id,
            notes,
        });

        self.current_state = to;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn current_state(&self) -> DocumentState {
        self.current_state
    }

    #[allow(dead_code)]
    pub fn audit_trail(&self) -> &[StateTransition] {
        &self.history
    }
}
