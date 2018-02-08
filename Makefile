readme:
	@echo "Building the README.md"
	@cat preamble.md > README.md
	@echo '```' >> README.md
	@# Run the test output, strip the colors, then put it in the README
	@cargo test -- --list | tee -a README.md
	@echo '```' >> README.md
	@echo "Done!"
