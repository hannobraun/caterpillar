module.exports = {
    // I have no idea which path this is relative to. Seemingly neither the
    // project root nor the `capi-web` directory.
    //
    // Anyway, as best as I can tell, the following over-general path specifier
    // seems to work.
    content: ["./**/src/**/*.rs"]
};
