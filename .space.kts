job("Build") {
	startOn {
		gitPush {
			anyTagMatching {
				+"v*"
			}
		}
	}
    container(displayName = "Build for musl", image = "atjontv/rust-musl:1.73.0") {
        shellScript {
            content = """
                set -e
                # Build for musl
                cargo build --verbose --release --target x86_64-unknown-linux-musl
            """
        }
		fileArtifacts {
			remotePath = "service_helper-musl.bin"
			localPath = "target/x86_64-unknown-linux-musl/release/service_helper"
		}
		cache {
			storeKey = "cargo_deps-{{ hashFiles('Cargo.toml', 'Cargo.lock') }}"
			localPath = "target/x86_64-unknown-linux-musl/release/deps"
            restoreKeys {
                +"cargo_deps-base"
            }
		}
    }
}
