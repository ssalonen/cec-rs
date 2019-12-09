# This script takes care of testing your crate

set -ex

# This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    #
    # test that example builds
    #
    CEC_RS_EXAMPLE_BUILD=$(mktemp -d -t cec-rs-example-build-XXXXXXXXX)
    function cleanup {
        rm -rf $CEC_RS_EXAMPLE_BUILD
    }
    trap cleanup EXIT
    echo "Building cec-rs example in $CEC_RS_EXAMPLE_BUILD"    
    for example in $(ls examples/); do        
        # Workaround Cross caveat of path dependencies pointing outside Cargo project
        # by copying sources to temporary directory
        cp -a examples/$example $CEC_RS_EXAMPLE_BUILD/$example
        mkdir $CEC_RS_EXAMPLE_BUILD/$example/cec_rs_src && cp -a {src,Cargo.toml,Cross.toml} $CEC_RS_EXAMPLE_BUILD/$example/cec_rs_src
        sed -i -E 's@cec-rs\s*=\s*\{\s*path\s*=\s*"../.."\s*\}@cec-rs = { path = "cec_rs_src" }@' $CEC_RS_EXAMPLE_BUILD/$example/Cargo.toml
        ( cd $CEC_RS_EXAMPLE_BUILD/$example/ \
          && cross check --target $TARGET )        
    done
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
