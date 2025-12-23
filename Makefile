FILE := Mustang-CLI-2.20.0.jar
FILE_NO_EXT := $(basename $(FILE))
FILE_REDUCED := $(FILE_NO_EXT)-reduced.jar

JBIN := /usr/local/opt/openjdk@17/bin
JRE_DIR := jre
JDK_HOME := /usr/local/opt/openjdk@17

GRAALVM_HOME := ~/opt/graalvm-jdk-25.0.1+8.1/Contents/Home
GRAALVM_BIN := $(GRAALVM_HOME)/bin
REFLECTCONFIG := manual_reflectconfig.json
GRAALVM_FLAGS := -H:+UnlockExperimentalVMOptions -H:+AddAllCharsets -H:ReflectionConfigurationFiles=$(REFLECTCONFIG) -H:ConfigurationFileDirectories=tracing-agent/combined/ -Djava.awt.headless=true 

PROGUARD_HOME := ~/opt/proguard-7.6.1

JDEPS = $(shell cat jdeps.txt)

.PHONY: clean run deps

$(JRE_DIR): jdeps.txt
	@echo "Creating custom JRE with jlink..."
	rm -rf $(JRE_DIR)
	$(JBIN)/jlink \
		--module-path $(JDK_HOME)/jmods \
		--add-modules $(JDEPS) \

		--output $(JRE_DIR) \
		--strip-debug \
		--no-man-pages \
		--no-header-files \
		--compress=2

run: $(JRE_DIR)
	$(JRE_DIR)/bin/java -jar $(FILE)

jdeps.txt: $(FILE)
	$(JBIN)/jdeps --ignore-missing-deps --print-module-deps $< > $@

deps: jdeps.txt
	cat $<

clean:
	rm -rf $(JRE_DIR)

$(FILE):
	curl -L -o $@ https://www.mustangproject.org/deploy/Mustang-CLI-2.20.0.jar

$(FILE_REDUCED): $(FILE) myconfig.pro
	export JAVA_HOME=$(JDK_HOME) && \
	$(PROGUARD_HOME)/bin/proguard.sh @myconfig.pro

print-main-class: $(FILE)
	unzip -p $< META-INF/MANIFEST.MF | grep 'Main-Class:' | cut -d' ' -f2

build-graalvm: Mustang-CLI-2.20.0

# without -Os, its about 72MB
# with -Os, its about 45MB
# with upx, its about 17MB (also fails to run)
$(FILE_NO_EXT): $(FILE) Makefile $(REFLECTCONFIG)
	$(GRAALVM_BIN)/native-image -Os $(GRAALVM_FLAGS) -jar $< -o $@

# upx is broken on macos right now
# $(FILE_NO_EXT)-upx: $(FILE_NO_EXT)
# 	upx -o $@ $<

# $(FILE_NO_EXT)-upx95: $(FILE_NO_EXT)
# 	upx -9 -o $@ $<

# without -Os, its about 62MB
$(FILE_NO_EXT)-reduced: $(FILE_REDUCED) Makefile $(REFLECTCONFIG)
	$(GRAALVM_BIN)/native-image $(GRAALVM_FLAGS) -jar $< -o $@

.PHONY: tracing-agent

tracing-agent:
	+$(MAKE) tracing-agent-raw-data
	+$(MAKE) tracing-agent/combined/reachability-metadata.json

tracing-agent-raw-data:
	rm -rf tracing-agent
	mkdir -p tracing-agent
	
	export JAVA_HOME=$(GRAALVM_HOME) && \
	export USE_TRACING_AGENT=true && \
	export USE_GRAALVM=false && \
	cargo test


$(GRAALVM_BIN)/native-image-configure:
	$(GRAALVM_BIN)/native-image --macro:native-image-configure-launcher

META-INF/native-image/reachability-metadata.json: tracing-agent/combined/reachability-metadata.json
	mkdir -p $(dir $@)
	cp $< $@

tracing-agent/combined/reachability-metadata.json: $(GRAALVM_BIN)/native-image-configure
	$(GRAALVM_BIN)/native-image-configure generate \
		$(shell find tracing-agent/ -type d -exec echo --input-dir={}/ \; | tr '\n' ' ') \
		--output-dir=tracing-agent/combined
	
test-graalvm:
	export USE_GRAALVM=true && \
	export USE_TRACING_AGENT=false && \
	cargo test


