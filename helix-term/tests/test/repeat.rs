use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn repeat_inside_word() -> anyhow::Result<()> {
    test((
        "#[|a]#pple banana cherry",
        "ciworange<esc>w.w.",
        "orange orange orang#[e|]#",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_undo() -> anyhow::Result<()> {
    test((
        "#[|a]#pple banana",
        "ciworange<esc>w.u",
        "orange #[b|]#anana",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_empty_replacement() -> anyhow::Result<()> {
    test(("#[|a]#pple banana", "ciw<esc>0l.", "#[ |]#")).await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_cancelled_operator() -> anyhow::Result<()> {
    test((
        "#[|a]#pple banana",
        "ciworange<esc>ci<esc>w.",
        "orange orang#[e|]#",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_after_yank() -> anyhow::Result<()> {
    test((
        "#[|a]#pple banana",
        "ciworange<esc>yiww.",
        "orange orang#[e|]#",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_replaced_by_insertion() -> anyhow::Result<()> {
    test((
        "#[|a]#pple banana",
        "ciworange<esc>wix<esc>l.",
        "orange x#[x|]#banana",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_whole_line() -> anyhow::Result<()> {
    test((
        "#[|a]#pple\nbanana\n",
        "ccorange<esc>j0.",
        "orange\norang#[e|]#\n",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_find_character() -> anyhow::Result<()> {
    test(("#[|a]#pple, banana,", "cf,x<esc>w.", "x #[x|]#")).await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_operator_count() -> anyhow::Result<()> {
    test((
        "#[|a]#pple\npear\nbanana\nplum\n",
        "2ccx<esc>j0.",
        "x\n#[x|]#\n",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_unicode() -> anyhow::Result<()> {
    test(("#[|é]#té 日本語", "ciw猫<esc>w.", "猫 #[猫|]#")).await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_latest_change() -> anyhow::Result<()> {
    test((
        "#[|a]#pple banana cherry",
        "ciwx<esc>wciworange<esc>w.",
        "x orange orang#[e|]#",
    ))
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_named_register() -> anyhow::Result<()> {
    let mut app = AppBuilder::default()
        .with_input_text("#[|a]#pple banana\n")
        .build()?;
    test_key_sequence(
        &mut app,
        Some("\"aciworange<esc>w."),
        Some(&|app| {
            let values: Vec<_> = app
                .editor
                .registers
                .read('a', &app.editor)
                .unwrap()
                .collect();
            assert_eq!(values, ["banana"]);
        }),
        false,
    )
    .await
}
