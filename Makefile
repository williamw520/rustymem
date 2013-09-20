
# Targets
CLIENT = rustymem
CLIENT_TEST = client_test

# Dir setup
ROOT_DIR        = .
SRC_DIR         = $(ROOT_DIR)/src
TEST_DIR        = $(ROOT_DIR)/src/test
BUILD_DIR       = $(ROOT_DIR)/bin
LIBRARY_DIRS    = $(BUILD_DIR)
#ROPTS           = --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS)
ROPTS           = --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS) --cfg debug

# Compile command, for general and for Windows
RUSTC          = rustc
#RUSTC           = rustc.exe


#all:  $(BUILD_DIR)/$(CLIENT)
all:  $(BUILD_DIR)/$(CLIENT_TEST)

$(BUILD_DIR)/$(BUILD_DIR).stamp:
	@echo "Building $@..."
	@mkdir -p $(BUILD_DIR)
	@touch $@

$(BUILD_DIR)/$(CLIENT).stamp: $(SRC_DIR)/$(CLIENT).rs  $(wildcard $(SRC_DIR)/rustymem_lib/*)  $(wildcard $(SRC_DIR)/common/*)  $(wildcard $(SRC_DIR)/common/**/*)  $(BUILD_DIR)/$(BUILD_DIR).stamp
	@echo "Building $<..."
	@$(RUSTC) $(ROPTS)  $<
	@touch $@

$(BUILD_DIR)/$(CLIENT_TEST): $(TEST_DIR)/$(CLIENT_TEST).rs  $(BUILD_DIR)/$(CLIENT).stamp  $(BUILD_DIR)/$(BUILD_DIR).stamp
	@echo "Building $@..."
	@$(RUSTC) $(ROPTS)  $<

clean:
	rm -R -f $(BUILD_DIR)
	rm -f $(SRC_DIR)/*~
	rm -f *~


scratch: $(BUILD_DIR)/scratch
	@$(RUSTC) --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS)  $(TEST_DIR)/scratch.rs
	@$(BUILD_DIR)/scratch

test-strutil:
	@$(RUSTC) --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS) --test $(SRC_DIR)/common/strutil.rs
	@$(BUILD_DIR)/strutil

test-netutil:
	@$(RUSTC) --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS) --test $(SRC_DIR)/common/netutil.rs
	@$(BUILD_DIR)/netutil

test-ioutil:
	@$(RUSTC) --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS) --test $(SRC_DIR)/common/ioutil.rs
	@$(BUILD_DIR)/ioutil

bench-client-test:
	@$(RUSTC) --out-dir=$(BUILD_DIR) -L $(LIBRARY_DIRS) --test $(SRC_DIR)/test/client_test.rs
	@$(BUILD_DIR)/client_test --bench

