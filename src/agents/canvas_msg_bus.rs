use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

/// CanvasSelectMsgBus
/// Transmits messages from the canvas component to the root component
/// and the config editor components

pub struct CanvasSelectMsgBus {
    link: AgentLink<CanvasSelectMsgBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for CanvasSelectMsgBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = CanvasMsgRequest;
    type Output = CanvasMsgRequest;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CanvasMsgRequest {
    FractalStarted,
    FractalProgress(String),
    FractalPaused,
    CanvasSelectMsg((u32, u32, u32, u32)),
}
