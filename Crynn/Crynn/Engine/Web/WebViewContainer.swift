import SwiftUI
import WebKit
import os.signpost

final class WebViewCoordinator: NSObject, WKNavigationDelegate, WKUIDelegate {
    @Binding var urlString: String
    let tabId: UUID
    weak var webView: WKWebView?
    weak var tabs: TabModel?
    private let blocker: ContentBlocker
    private let tracer = PerformanceTracer.shared
    private var isThrottled = false

    init(urlString: Binding<String>, tabId: UUID, blocker: ContentBlocker, tabs: TabModel?) {
        _urlString = urlString
        self.tabId = tabId
        self.blocker = blocker
        self.tabs = tabs
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        tracer.pageFinished(tabId: tabId)
    }

    func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
        tracer.pageStarted(tabId: tabId)
    }

    func throttle(active: Bool) {
        guard let wv = webView, isThrottled != !active else { return }
        isThrottled = !active
        let js = active ? """
            (function(){if(window._crynnThrottle){clearInterval(window._crynnThrottle);delete window._crynnThrottle}})();
        """ : """
            (function(){
                var t=setInterval(function(){},1000);
                if(window._crynnThrottle)clearInterval(window._crynnThrottle);
                window._crynnThrottle=t;
            })();
        """
        wv.evaluateJavaScript(js) { _, _ in }
    }
}

struct WebViewContainer: NSViewRepresentable {
    @Binding var urlString: String
    @Binding var isReader: Bool
    let tabId: UUID
    @EnvironmentObject private var blocker: ContentBlocker
    @EnvironmentObject private var tabs: TabModel

    func makeNSView(context: Context) -> WKWebView {
        WebContext.shared.prewarmIfNeeded(blocker: blocker)
        let webView = WebContext.shared.acquireWebView(blocker: blocker)
        webView.customUserAgent = nil
        webView.navigationDelegate = context.coordinator
        webView.uiDelegate = context.coordinator
        context.coordinator.webView = webView
        context.coordinator.tabs = tabs
        let isActive = tabs.activeTabID == tabId
        context.coordinator.throttle(active: isActive)
        load(urlString: urlString, in: webView)
        return webView
    }

    func updateNSView(_ webView: WKWebView, context: Context) {
        let isActive = tabs.activeTabID == tabId
        context.coordinator.throttle(active: isActive)
        
        // Check if URL needs to be loaded - compare normalized URLs
        let currentURL = webView.url?.absoluteString ?? ""
        let normalizedCurrent = currentURL.lowercased().trimmingCharacters(in: .whitespaces)
        let normalizedNew = urlString.lowercased().trimmingCharacters(in: .whitespaces)
        
        if normalizedCurrent != normalizedNew && !urlString.isEmpty && urlString != "about:blank" {
            load(urlString: urlString, in: webView)
        }
        
        // Update content blocking per-site
        let host = webView.url?.host
        let uc = webView.configuration.userContentController
        uc.removeAllContentRuleLists()
        if blocker.isEnabled(for: host), let list = blocker.currentRuleList {
            uc.add(list)
        }
        if isReader { ReaderSimplifier.applySimplify(in: webView) }
    }

    static func dismantleNSView(_ webView: WKWebView, coordinator: WebViewCoordinator) {
        coordinator.throttle(active: false)
        WebContext.shared.releaseWebView(webView)
    }

    func makeCoordinator() -> WebViewCoordinator {
        WebViewCoordinator(urlString: $urlString, tabId: tabId, blocker: blocker, tabs: tabs)
    }

    private func load(urlString: String, in webView: WKWebView) {
        let trimmed = urlString.trimmingCharacters(in: .whitespaces)
        guard !trimmed.isEmpty, trimmed != "about:blank" else { return }
        
        let req: URLRequest
        // Try to parse as URL first
        if let url = URL(string: trimmed), url.scheme != nil {
            req = URLRequest(url: url, cachePolicy: .returnCacheDataElseLoad, timeoutInterval: 30)
        } else if trimmed.contains(".") && !trimmed.contains(" ") {
            // If it looks like a domain (has dots, no spaces), add https://
            if let url = URL(string: "https://\(trimmed)") {
                req = URLRequest(url: url, cachePolicy: .returnCacheDataElseLoad, timeoutInterval: 30)
            } else { return }
        } else {
            // Treat as search query
            guard let encoded = trimmed.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed),
                  let url = URL(string: "https://duckduckgo.com/?q=\(encoded)") else { return }
            req = URLRequest(url: url)
        }
        
        PerformanceTracer.shared.pageRequested(tabId: tabId)
        webView.load(req)
    }
}


