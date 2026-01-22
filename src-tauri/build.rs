fn main() {
    tauri_build::build();

    // TODO: 启用 tauri-specta 类型同步
    // 由于 RC 版本的 API 变化，暂时禁用自动类型生成
    // 后续可以手动维护 TypeScript 类型定义
    /*
    tauri_specta::ts::export(
        specta::collect_types![
            specta::DynamicNode::from::<arg_forge_lib::Project>(),
            specta::DynamicNode::from::<arg_forge_lib::Form>(),
            specta::DynamicNode::from::<arg_forge_lib::FormItem>(),
            specta::DynamicNode::from::<arg_forge_lib::ParamStyle>(),
        ]
        .unwrap(),
        "../src/types/bindings.ts",
    )
    .expect("Failed to export types");
    */
}

