// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetABIEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SetABIEvent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetABIEvent {
    /// transaction
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_ordinal: u32,
    /// contract & action
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub action: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="5")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub abi: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
