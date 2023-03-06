use lindera::DictionaryKind;
use lindera::mode::Mode;
use lindera::tokenizer::{DictionaryConfig, TokenizerConfig};

pub struct CustomTokenizer {
    pub tokenizer: lindera::tokenizer::Tokenizer
}

impl CustomTokenizer {
    pub fn new() -> CustomTokenizer {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };

        CustomTokenizer {
            // TODO: resultのエラーハンドリング
            tokenizer: lindera::tokenizer::Tokenizer::from_config(config)?
        }
    }
}