const FABRIC_API: &str = "https://cdn.modrinth.com/data/P7dR8mSH/versions/hfsU4hXq/fabric-api-0.76.0%2B1.19.2.jar";
const SIMPLE_VOICE_CHAT: &str = "https://cdn.modrinth.com/data/9eGKb6K1/versions/K95RbSbU/voicechat-fabric-1.19.2-2.4.16.jar";
const PHOSPHOR: &str = "https://cdn.modrinth.com/data/hEOCdOgW/versions/mc1.19.x-0.8.1/phosphor-fabric-mc1.19.x-0.8.1.jar";
const LITHIUM: &str = "https://cdn.modrinth.com/data/gvQqBUqZ/versions/m6sVgAi6/lithium-fabric-mc1.19.2-0.11.1.jar";
const IRIS: &str = "https://cdn.modrinth.com/data/YL57xq9U/versions/9YEwbzW6/iris-mc1.19.2-1.6.4.jar";
const SODIUM: &str = "https://cdn.modrinth.com/data/AANobbMI/versions/rAfhHfow/sodium-fabric-mc1.19.2-0.4.4%2Bbuild.18.jar";
const CREATE: &str = "https://cdn.modrinth.com/data/Xbc0uyRg/versions/wKEEi1qX/create-fabric-0.5.1-b-build.1089%2Bmc1.19.2.jar";
const AE2: &str = "https://cdn.modrinth.com/data/XxWD5pD3/versions/lePuKDdy/appliedenergistics2-fabric-12.9.6.jar";
const XAEROS_MINIMAP: &str = "https://cdn.modrinth.com/data/1bokaNcj/versions/LKcX5Que/Xaeros_Minimap_23.6.0_Fabric_1.19.1.jar";
const XAEROS_WORLD_MAP: &str = "https://cdn.modrinth.com/data/NcUtCpym/versions/st6Yi8FJ/XaerosWorldMap_1.31.0_Fabric_1.19.1.jar";
const INDIUM: &str = "https://cdn.modrinth.com/data/Orvt0mRa/versions/yTfou6df/indium-1.0.9%2Bmc1.19.2.jar";
const JEI: &str = "https://cdn.modrinth.com/data/u6dRKJwZ/versions/8y6r09NZ/jei-1.19.2-fabric-11.6.0.1016.jar";
const LAZY_DFU: &str = "https://cdn.modrinth.com/data/hvFnDODi/versions/0.1.3/lazydfu-0.1.3.jar";
const FERRITE_CORE: &str = "https://cdn.modrinth.com/data/uXXizFIs/versions/kwjHqfz7/ferritecore-5.0.3-fabric.jar";
const CLUMPS: &str = "https://cdn.modrinth.com/data/Wnxd13zP/versions/3GURrv52/Clumps-forge-1.19.2-9.0.0%2B14.jar";
//const DASH_LOADER: &str = "https://cdn.modrinth.com/data/ZfQ3kTvR/versions/DsPMHgmj/dashloader-4.1.3%2B1.19.jar";

pub fn get_mod_list() -> Vec<String> {
    let mods = vec![
        FABRIC_API.into(),
        SIMPLE_VOICE_CHAT.into(),
        PHOSPHOR.into(),
        LITHIUM.into(),
        IRIS.into(),
        SODIUM.into(),
        CREATE.into(),
        AE2.into(),
        XAEROS_WORLD_MAP.into(),
        XAEROS_MINIMAP.into(),
        INDIUM.into(),
        JEI.into(),
        LAZY_DFU.into(),
        FERRITE_CORE.into(),
        CLUMPS.into(),
        //DASH_LOADER.into(),
    ];

    mods
}
