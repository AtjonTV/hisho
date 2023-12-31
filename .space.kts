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
			remotePath = "hisho-musl.bin"
			localPath = "target/x86_64-unknown-linux-musl/release/hisho_cli2"
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

job("Test") {
    container(displayName = "Test for musl", image = "atjontv/rust-musl-sccache:1.73.0-3") {
        mountDir = "/root"
        shellScript {
            content = """
                set -e
                # Test for musl
                cargo test --all --all-features --verbose --release --target x86_64-unknown-linux-musl
            """
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
