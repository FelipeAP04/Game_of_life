# This file will be configured to contain variables for CPack. These variables
# should be set in the CMake list file of the project before CPack module is
# included. The list of available CPACK_xxx variables and their associated
# documentation may be obtained using
#  cpack --help-variable-list
#
# Some variables are common to all generators (e.g. CPACK_PACKAGE_NAME)
# and some are specific to a generator
# (e.g. CPACK_NSIS_EXTRA_INSTALL_COMMANDS). The generator specific variables
# usually begin with CPACK_<GENNAME>_xxxx.


set(CPACK_BINARY_7Z "")
set(CPACK_BINARY_BUNDLE "")
set(CPACK_BINARY_CYGWIN "")
set(CPACK_BINARY_DEB "")
set(CPACK_BINARY_DRAGNDROP "")
set(CPACK_BINARY_FREEBSD "")
set(CPACK_BINARY_IFW "")
set(CPACK_BINARY_NSIS "")
set(CPACK_BINARY_NUGET "")
set(CPACK_BINARY_OSXX11 "")
set(CPACK_BINARY_PACKAGEMAKER "")
set(CPACK_BINARY_PRODUCTBUILD "")
set(CPACK_BINARY_RPM "")
set(CPACK_BINARY_STGZ "")
set(CPACK_BINARY_TBZ2 "")
set(CPACK_BINARY_TGZ "")
set(CPACK_BINARY_TXZ "")
set(CPACK_BINARY_TZ "")
set(CPACK_BINARY_WIX "")
set(CPACK_BINARY_ZIP "")
set(CPACK_BUILD_SOURCE_DIRS "/home/lipito/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raylib-sys-5.5.1/raylib;/home/lipito/game_of_life/target/release/build/raylib-sys-b1e66d2e47599788/out/build")
set(CPACK_CMAKE_GENERATOR "Unix Makefiles")
set(CPACK_COMPONENT_UNSPECIFIED_HIDDEN "TRUE")
set(CPACK_COMPONENT_UNSPECIFIED_REQUIRED "TRUE")
set(CPACK_DEBIAN_PACKAGE_DEPENDS "libatomic1, libc6, libglfw3, libglu1-mesa | libglu1, libglx0, libopengl0")
set(CPACK_DEBIAN_PACKAGE_NAME "libraylib-dev")
set(CPACK_DEBIAN_PACKAGE_SHLIBDEPS "OFF")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_FILE "/usr/share/cmake-3.16/Templates/CPack.GenericDescription.txt")
set(CPACK_GENERATOR "ZIP;TGZ;DEB;RPM")
set(CPACK_INSTALL_CMAKE_PROJECTS "/home/lipito/game_of_life/target/release/build/raylib-sys-b1e66d2e47599788/out/build;raylib;ALL;/")
set(CPACK_INSTALL_PREFIX "/home/lipito/game_of_life/target/release/build/raylib-sys-b1e66d2e47599788/out")
set(CPACK_MODULE_PATH "/home/lipito/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raylib-sys-5.5.1/raylib/cmake")
set(CPACK_NSIS_DISPLAY_NAME "raylib 5.5.0")
set(CPACK_NSIS_INSTALLER_ICON_CODE "")
set(CPACK_NSIS_INSTALLER_MUI_ICON_CODE "")
set(CPACK_NSIS_INSTALL_ROOT "$PROGRAMFILES")
set(CPACK_NSIS_PACKAGE_NAME "raylib 5.5.0")
set(CPACK_OUTPUT_CONFIG_FILE "/home/lipito/game_of_life/target/release/build/raylib-sys-b1e66d2e47599788/out/build/CPackConfig.cmake")
set(CPACK_PACKAGE_CONTACT "raysan5")
set(CPACK_PACKAGE_DEFAULT_LOCATION "/")
set(CPACK_PACKAGE_DESCRIPTION_FILE "/home/lipito/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raylib-sys-5.5.1/raylib/src/../README.md")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Simple and easy-to-use library to enjoy videogames programming")
set(CPACK_PACKAGE_FILE_NAME "raylib-5.5.0")
set(CPACK_PACKAGE_INSTALL_DIRECTORY "raylib 5.5.0")
set(CPACK_PACKAGE_INSTALL_REGISTRY_KEY "raylib 5.5.0")
set(CPACK_PACKAGE_NAME "raylib")
set(CPACK_PACKAGE_RELOCATABLE "true")
set(CPACK_PACKAGE_VENDOR "Humanity")
set(CPACK_PACKAGE_VERSION "5.5.0")
set(CPACK_PACKAGE_VERSION_MAJOR "")
set(CPACK_PACKAGE_VERSION_MINOR "")
set(CPACK_PACKAGE_VERSION_PATCH "")
set(CPACK_RESOURCE_FILE_LICENSE "/home/lipito/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raylib-sys-5.5.1/raylib/src/../LICENSE")
set(CPACK_RESOURCE_FILE_README "/usr/share/cmake-3.16/Templates/CPack.GenericDescription.txt")
set(CPACK_RESOURCE_FILE_WELCOME "/home/lipito/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raylib-sys-5.5.1/raylib/src/../README.md")
set(CPACK_RPM_PACKAGE_NAME "libraylib-devel")
set(CPACK_SET_DESTDIR "OFF")
set(CPACK_SOURCE_7Z "")
set(CPACK_SOURCE_CYGWIN "")
set(CPACK_SOURCE_GENERATOR "TBZ2;TGZ;TXZ;TZ")
set(CPACK_SOURCE_OUTPUT_CONFIG_FILE "/home/lipito/game_of_life/target/release/build/raylib-sys-b1e66d2e47599788/out/build/CPackSourceConfig.cmake")
set(CPACK_SOURCE_RPM "OFF")
set(CPACK_SOURCE_TBZ2 "ON")
set(CPACK_SOURCE_TGZ "ON")
set(CPACK_SOURCE_TXZ "ON")
set(CPACK_SOURCE_TZ "ON")
set(CPACK_SOURCE_ZIP "OFF")
set(CPACK_SYSTEM_NAME "Linux")
set(CPACK_TOPLEVEL_TAG "Linux")
set(CPACK_WIX_SIZEOF_VOID_P "8")

if(NOT CPACK_PROPERTIES_FILE)
  set(CPACK_PROPERTIES_FILE "/home/lipito/game_of_life/target/release/build/raylib-sys-b1e66d2e47599788/out/build/CPackProperties.cmake")
endif()

if(EXISTS ${CPACK_PROPERTIES_FILE})
  include(${CPACK_PROPERTIES_FILE})
endif()
