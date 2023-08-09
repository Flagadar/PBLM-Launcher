pub fn get_fabric_libs() -> Vec<String> {
    let libs = vec![
        "https://maven.fabricmc.net/net/fabricmc/tiny-mappings-parser/0.3.0+build.17/tiny-mappings-parser-0.3.0+build.17.jar".into(),
        "https://maven.fabricmc.net/net/fabricmc/sponge-mixin/0.12.5+mixin.0.8.5/sponge-mixin-0.12.5+mixin.0.8.5.jar".into(),
        "https://maven.fabricmc.net/net/fabricmc/tiny-remapper/0.8.2/tiny-remapper-0.8.2.jar".into(),
        "https://maven.fabricmc.net/net/fabricmc/access-widener/2.1.0/access-widener-2.1.0.jar".into(),
        "https://maven.fabricmc.net/org/ow2/asm/asm/9.5/asm-9.5.jar".into(),
        "https://maven.fabricmc.net/org/ow2/asm/asm-analysis/9.5/asm-analysis-9.5.jar".into(),
        "https://maven.fabricmc.net/org/ow2/asm/asm-commons/9.5/asm-commons-9.5.jar".into(),
        "https://maven.fabricmc.net/org/ow2/asm/asm-tree/9.5/asm-tree-9.5.jar".into(),
        "https://maven.fabricmc.net/org/ow2/asm/asm-util/9.5/asm-util-9.5.jar".into(),
        "https://maven.fabricmc.net/net/fabricmc/intermediary/1.19.2/intermediary-1.19.2.jar".into(),
        "https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.14.21/fabric-loader-0.14.21.jar".into()
    ];

    libs
}
