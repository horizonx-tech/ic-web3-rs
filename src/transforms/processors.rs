use super::transform::{
    ArrayResultTransformProcessor, ArrayResultTransformProcessorBuilder, SingleResultTransformProcessor,
    SingleResultTransformProcessorBuilder,
};

pub fn send_transaction_processor() -> SingleResultTransformProcessor {
    SingleResultTransformProcessorBuilder::default()
        .transaction_index(true)
        .build()
        .unwrap()
}

pub fn get_filter_changes_processor() -> ArrayResultTransformProcessor {
    ArrayResultTransformProcessorBuilder::default()
        .log_index(true)
        .transaction_index(true)
        .build()
        .unwrap()
}
