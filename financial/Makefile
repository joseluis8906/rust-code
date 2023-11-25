open=xdg-open

.PHONY: test
test:
	@bazel test //financial/src:lib_test --test_output=errors

.PHONY: cov
cov:
	@bazel coverage --combined_report=lcov //financial/src:lib_test
	@cd .. && genhtml --no-branch-coverage -output genhtml "$$(bazel info output_path)/k8-fastbuild/testlogs/financial/src/lib_test/coverage.dat"
	@cd .. && $(open) ./genhtml/index.html
