job("Build") {
	startOn {
		gitPush {
			anyTagMatching {
				+"v*"
			}
		}
	}
    container(displayName = "Build for musl", image = "atjontv/rust-musl-sccache:1.73.0-3") {
        mountDir = "/root"
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
			storeKey = "sccache-{{ hashFiles('.cache.lock') }}"
			localPath = "/root/.cache/sccache"
		}
        cache {
            storeKey = "cargo-{{ hashFiles('.cache.lock') }}"
            localPath = "/root/.cargo"
        }

    }
}
