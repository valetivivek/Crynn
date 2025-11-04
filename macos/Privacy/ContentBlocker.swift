import Foundation
import Combine
import WebKit

final class ContentBlocker: ObservableObject {
    @Published var enabled: Bool = true { didSet { loadRules() } }
    @Published var disabledHosts: Set<String> = []
    private(set) var currentRuleList: WKContentRuleList?

    init() { loadRules() }

    func loadRules() {
        guard enabled else { currentRuleList = nil; return }
        guard let store = WKContentRuleListStore.default() else { return }
        let rules = Self.compactRules
        store.compileContentRuleList(forIdentifier: "crynn.rules", encodedContentRuleList: rules) { [weak self] list, _ in
            DispatchQueue.main.async { self?.currentRuleList = list }
        }
    }

    func isEnabled(for host: String?) -> Bool {
        guard enabled else { return false }
        guard let h = host?.lowercased() else { return true }
        return !disabledHosts.contains(h)
    }

    func toggleSite(host: String?) {
        guard let h = host?.lowercased(), !h.isEmpty else { return }
        if disabledHosts.contains(h) { disabledHosts.remove(h) } else { disabledHosts.insert(h) }
        objectWillChange.send()
    }

    func clearAllWebsiteData() {
        let types = WKWebsiteDataStore.allWebsiteDataTypes()
        WKWebsiteDataStore.default().removeData(ofTypes: types, modifiedSince: .distantPast) { }
    }

    private static let compactRules = """
    [
      {"trigger":{"url-filter":".*","resource-type":["popup"]},"action":{"type":"block"}},
      {"trigger":{"url-filter":"(^https?:\\/\\/)?([a-z0-9-]+\\.)*(doubleclick|googlesyndication|adservice|adsystem|taboola|outbrain|scorecardresearch|criteo|quantserve|mathtag|rubiconproject)\\.com"},"action":{"type":"block"}},
      {"trigger":{"url-filter":"(^https?:\\/\\/)?([a-z0-9-]+\\.)*(analytics|ga)\\.googleapis\\.com"},"action":{"type":"block"}}
    ]
    """
}


