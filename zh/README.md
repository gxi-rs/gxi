<h1 align="center">GXI</h1>

<p align="center">
    <i>声明式跨平台原生 UI 框架</i>
</p>

<p align="center"><a href="https://github.com/gxi-rs/gxi/actions"></a><img src="https://github.com/gxi-rs/gxi/actions/workflows/tests.yml/badge.svg" alt="测试"> <a href="https://crates.io/crates/gxi"></a><img src="https://img.shields.io/crates/v/gxi" alt="板条箱.io"></p>

> ⚠️ 早期

使用[proc-macros](https://doc.rust-lang.org/reference/procedural-macros.html) ， [gxi 转译](gxi-transpiler/README.md)器使用可观察的同步/异步状态模式将组件树转译为有状态的自我管理的 n 二进制树，以实现最高效率和最低可能的开销，几乎没有运行时成本。本质上，消除了[虚拟 dom](https://reactjs.org/docs/faq-internals.html)或[差异算法](https://reactjs.org/docs/reconciliation.html)的使用。组件系统与平台无关，它允许系统生成平台相关和独立的组件，将它们合并以实现代码重用和可维护性。

## 平台

- [x] Web `wasm32-unknown-unknown`
- [ ] 桌面 (GTK) Windows、Mac 和 Linux
- [ ] 平台无关（Web 和 GTK）
- [ ] 安卓
- [] IOS

## 例子

```rust
use gxi::{gxi, set_state, State, StrongNodeType, Text};

pub fn cat_fact() -> StrongNodeType {
    let cat_fact = State::new(String::new());

    let fetch_cat_fact = set_state!(
        async || {
            let resp = reqwest::get("https://catfact.ninja/fact?max_length=140")
                .await
                .unwrap();
            *(*cat_fact).borrow_mut() = resp.text().await.unwrap();
        },
        [cat_fact]
    );

    // pre fetch cat memes
    fetch_cat_fact();

    return gxi! {
        div [
            button ( on_click = move |_| fetch_cat_fact() ) [
                Text ( value = "fetch cat memes!" )
            ],
            p [
                Text ( value = &cat_fact[..])
            ],
        ]
    };
}
```

![](./gxi-web-eg.gif)

完整的 src[在这里](examples)

## 行为守则

行为准则可在**[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)**中找到

## 贡献

确保在贡献之前阅读**[贡献指南](CONTRIBUTING.md)**。

## 许可和版权

版权所有 (C) 2021 Aniket Prajapati

根据**[MIT 许可证获得许可](LICENSE)**

## 贡献者

- [Aniket Prajapati](https://aniketprajapati.me) &lt; [contact@aniketprajapati.me](mailto:contact@aniketprajapati.me) &gt;
