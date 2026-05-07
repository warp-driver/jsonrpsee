macro_rules! cfg_feature {
	($feature:literal, $($item:item)*) => {
		$(
			#[cfg(feature = $feature)]
			#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
			$item
		)*
	}
}

macro_rules! cfg_client {
	($($item:item)*) => {
		$(
			#[cfg(any(
				feature = "wasi-jsonrpsee-http-client", feature = "wasi-jsonrpsee-wasm-client", feature = "wasi-jsonrpsee-ws-client",
				feature = "client", feature = "async-client", feature = "client-core", feature = "async-wasm-client"
			))]
			$item
		)*
	}
}

macro_rules! cfg_http_client {
	($($item:item)*) => {
		cfg_feature!("wasi-jsonrpsee-http-client", $($item)*);
	};
}

macro_rules! cfg_ws_client {
	($($item:item)*) => {
		cfg_feature!("wasi-jsonrpsee-ws-client", $($item)*);
	};
}

macro_rules! cfg_wasm_client {
	($($item:item)*) => {
		cfg_feature!("wasi-jsonrpsee-wasm-client", $($item)*);
	};
}

macro_rules! cfg_async_client {
	($($item:item)*) => {
		cfg_feature!("async-client", $($item)*);
	};
}

macro_rules! cfg_client_transport {
	($($item:item)*) => {
		cfg_feature!("wasi-jsonrpsee-client-transport", $($item)*);
	};
}

macro_rules! cfg_server {
	($($item:item)*) => {
		cfg_feature!("server", $($item)*);
	}
}

macro_rules! cfg_server_core {
	($($item:item)*) => {
		cfg_feature!("server-core", $($item)*);
	}
}

macro_rules! cfg_proc_macros {
	($($item:item)*) => {
		cfg_feature!("wasi-jsonrpsee-proc-macros", $($item)*);
	};
}

macro_rules! cfg_types {
	($($item:item)*) => {
		cfg_feature!("wasi-jsonrpsee-types", $($item)*);
	};
}

macro_rules! cfg_client_or_server {
	($($item:item)*) => {
		$(
			#[cfg(any(
				feature = "wasi-jsonrpsee-http-client", feature = "wasi-jsonrpsee-wasm-client", feature = "wasi-jsonrpsee-ws-client",
				feature = "client", feature = "async-client", feature = "async-wasm-client",
				feature = "client-core", feature = "server", feature = "server-core"
			))]
			$item
		)*
	}
}
