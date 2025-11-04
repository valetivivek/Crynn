import Foundation
import Combine

final class TabModel: ObservableObject {
    struct Tab: Identifiable, Equatable, Codable {
        let id: UUID
        var urlString: String
        var title: String
        var isPinned: Bool
        var isReader: Bool
        init(id: UUID = UUID(), urlString: String = "about:blank", title: String = "New Tab", isPinned: Bool = false, isReader: Bool = false) {
            self.id = id
            self.urlString = urlString
            self.title = title
            self.isPinned = isPinned
            self.isReader = isReader
        }
    }

    @Published var tabs: [Tab] = [Tab()]
    @Published var activeTabID: UUID = UUID()

    init() {
        activeTabID = tabs.first!.id
    }

    var activeIndex: Int? { tabs.firstIndex(where: { $0.id == activeTabID }) }
    var activeTab: Tab? {
        get { tabs.first(where: { $0.id == activeTabID }) }
        set {
            guard let newValue, let idx = activeIndex else { return }
            tabs[idx] = newValue
        }
    }

    func newTab(urlString: String = "about:blank", switchTo: Bool = true) {
        let tab = Tab(urlString: urlString)
        tabs.append(tab)
        PerformanceTracer.shared.tabCreated(id: tab.id)
        if switchTo { activeTabID = tab.id }
    }

    func duplicateActive() {
        guard let current = activeTab else { return }
        let dup = Tab(urlString: current.urlString, title: current.title, isPinned: current.isPinned)
        if let idx = activeIndex { tabs.insert(dup, at: idx + 1); activeTabID = dup.id }
    }

    func closeActiveTab() {
        guard let idx = activeIndex else { return }
        PerformanceTracer.shared.tabClosed(id: tabs[idx].id)
        tabs.remove(at: idx)
        if tabs.isEmpty { tabs = [Tab()]; activeTabID = tabs[0].id }
        else { activeTabID = tabs[min(idx, tabs.count - 1)].id }
    }

    func setURL(_ urlString: String, for id: UUID) {
        guard let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
        tabs[idx].urlString = urlString
    }

    func setTitle(_ title: String, for id: UUID) {
        guard let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
        tabs[idx].title = title
    }

    func togglePinned(for id: UUID) {
        guard let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
        tabs[idx].isPinned.toggle()
    }

    func toggleReader(for id: UUID) {
        guard let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
        tabs[idx].isReader.toggle()
    }
}



