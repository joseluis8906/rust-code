.PHONY: gazelle
gazelle:
	@bazel run //:gazelle

.PHONY: clean
clean:
	@bazel clean --expunge

.PHONY: run
run:
	@npx tailwindcss -i ./input.css -o ./style/output.css
	@cargo leptos watch
