//! Converts agent messages into attributed input while preserving their wake mode.

use crate::agent::types::AgentMessage;
use crate::agent::types::MessageDeliveryMode;
use crate::context::ContextualUserFragment;
use crate::context::InterAgentMessage;
use crate::context::InterAgentMessageType;
use codex_features::MultiAgentMessageDelivery;
use codex_protocol::AgentPath;
use codex_protocol::protocol::InterAgentCommunication;

impl AgentMessage {
    pub(crate) fn into_communication(
        self,
        author: AgentPath,
        recipient: AgentPath,
        mode: MessageDeliveryMode,
        message_delivery: MultiAgentMessageDelivery,
    ) -> InterAgentCommunication {
        let trigger_turn = mode == MessageDeliveryMode::TriggerTurn;
        match self {
            Self::Encrypted(message) => InterAgentCommunication::new_encrypted(
                author,
                recipient,
                Vec::new(),
                message,
                trigger_turn,
            ),
            Self::Plaintext(message) => {
                let message_type = match mode {
                    MessageDeliveryMode::QueueOnly => InterAgentMessageType::Message,
                    MessageDeliveryMode::TriggerTurn => InterAgentMessageType::NewTask,
                };
                let content = InterAgentMessage::new(
                    message_type,
                    recipient.clone(),
                    author.clone(),
                    message,
                )
                .render()
                .into_text()
                .unwrap_or_default();
                // The delivery policy is stamped onto the communication so the child's history
                // (and any replay of it) renders the same item type it originally received.
                match message_delivery {
                    MultiAgentMessageDelivery::Plaintext => {
                        InterAgentCommunication::new_portable_plaintext(
                            author,
                            recipient,
                            Vec::new(),
                            content,
                            trigger_turn,
                        )
                    }
                    MultiAgentMessageDelivery::Encrypted => {
                        InterAgentCommunication::new(author, recipient, Vec::new(), content, trigger_turn)
                    }
                }
            }
        }
    }
}
