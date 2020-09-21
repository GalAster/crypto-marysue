const locale_cn = {
    selectText: '选择语言',
    label: '简体中文',
    editLinkText: '在 GitHub 上编辑此页',
    sidebar: [
        {
            path: '/trivial/',
            title: 'Trivial',
            collapsable: false,
            sidebarDepth: 1,
            children: [
                ['caesar', '凯撒密码'],
                ['morse', '莫斯密码'],
            ]
        },
        {
            path: '/bizarre/',
            title: 'Bizarre',
            collapsable: false,
            sidebarDepth: 1,
            children: [
                ['brainfuck', 'Brainfuck']
                ['marysue', 'Marysue'],
            ]
        },
    ]
}

module.exports = {
    dest: 'docs/.build',
    head: [
        ['link', { rel: 'shortcut icon', type: "image/x-icon", href: './favicon.png' }],
        ['script',
            {
                type: '"text/javascript" async',
                src: 'https://unpkg.com/wasm-crypto-moe/wasm_crypto_moe.js'
            }
        ],
    ],
    themeConfig: {
        repo: 'GalAster/crypto-moe',
        editLinks: true,
        docsDir: 'projects/wasm-vuepress',
        markdown: {
            lineNumbers: true
        },
        ...locale_cn
    },
    markdown: {
        config: md => {
        }
    },
    plugins: {
        mathjax: {
            target: 'chtml',
            presets: [],
        },
        '@vuepress/pwa': {
            serviceWorker: true,
            updatePopup: true,
            popupComponent: 'PWAUpdate',
            generateSWConfig: {
                importWorkboxFrom: 'local'
            }
        }
    }
};
