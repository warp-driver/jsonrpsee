use wasi_jsonrpsee::proc_macros::rpc;

// Method without type marker, `#[method(…)]` or `#[subscription(…)]`.
#[rpc(client, server)]
pub trait NotQualified {
	async fn async_method(&self) -> wasi_jsonrpsee::core::RpcResult<u8>;
}

fn main() {}
