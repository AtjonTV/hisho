job("Build") {
	startOn {
		gitPush {
			anyTagMatching {
				+"v*"
			}
		}
	}
    container(displayName = "Run script", image = "rustlang/rust:nightly") {
        shellScript {
            content = """
                set -e
                rustup target add x86_64-unknown-linux-musl
                # Build the Rust project
                cargo build --verbose --release --target x86_64-unknown-linux-musl
            """
        }
		fileArtifacts {
			remotePath = "service_helper-musl.bin"
			localPath = "target/x86_64-unknown-linux-musl/release/service_helper"
		}
		cache {
			storeKey = "cargo-{{ hashFiles('Cargo.lock') }}"
			localPath = "target"
		}
    }
}
