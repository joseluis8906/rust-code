.PHONY: gazelle
gazelle:
	@bazel run //:gazelle

.PHONY: clean
clean:
	@bazel clean --expunge

.PHONY: run
run:
	@cargo leptos watch
