import WebKit

final class WebContext {
    static let shared = WebContext()

    @available(macOS, deprecated: 12.0, message: "WKProcessPool is deprecated but still functional")
    private let processPool = WKProcessPool()
    private(set) var prewarmed: WKWebView?
    private var reusePool: [WKWebView] = []
    private let maxPoolSize = 2

    func makeConfiguration(userContentController: WKUserContentController? = nil) -> WKWebViewConfiguration {
        let cfg = WKWebViewConfiguration()
        cfg.processPool = processPool
        cfg.websiteDataStore = .default()
        cfg.preferences.javaScriptCanOpenWindowsAutomatically = false
        cfg.preferences.isTextInteractionEnabled = true
        cfg.defaultWebpagePreferences.preferredContentMode = .recommended
        cfg.mediaTypesRequiringUserActionForPlayback = .all
        cfg.allowsAirPlayForMediaPlayback = false
        if let uc = userContentController { cfg.userContentController = uc }
        return cfg
    }

    func prewarmIfNeeded(blocker: ContentBlocker) {
        guard prewarmed == nil else { return }
        let uc = WKUserContentController()
        if let list = blocker.currentRuleList { uc.add(list) }
        prewarmed = WKWebView(frame: .zero, configuration: makeConfiguration(userContentController: uc))
        prewarmed?.isHidden = true
    }

    func acquireWebView(blocker: ContentBlocker) -> WKWebView {
        if let reused = reusePool.popLast() {
            return reused
        }
        let uc = WKUserContentController()
        if blocker.isEnabled(for: nil), let list = blocker.currentRuleList { uc.add(list) }
        return WKWebView(frame: .zero, configuration: makeConfiguration(userContentController: uc))
    }

    func releaseWebView(_ webView: WKWebView) {
        guard reusePool.count < maxPoolSize else { return }
        webView.stopLoading()
        webView.loadHTMLString("", baseURL: nil)
        reusePool.append(webView)
    }
}


