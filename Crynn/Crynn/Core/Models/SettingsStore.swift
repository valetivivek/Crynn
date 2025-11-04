import Foundation
import Combine

final class SettingsStore: ObservableObject {
    enum SearchEngine: String, CaseIterable, Identifiable { case duckduckgo, google, bing; var id: String { rawValue } }
    @Published var searchEngine: SearchEngine { didSet { UserDefaults.standard.set(searchEngine.rawValue, forKey: "searchEngine") } }
    @Published var homepage: String { didSet { UserDefaults.standard.set(homepage, forKey: "homepage") } }

    init() {
        let raw = UserDefaults.standard.string(forKey: "searchEngine") ?? SearchEngine.duckduckgo.rawValue
        searchEngine = SearchEngine(rawValue: raw) ?? .duckduckgo
        homepage = UserDefaults.standard.string(forKey: "homepage") ?? "about:blank"
    }
}



