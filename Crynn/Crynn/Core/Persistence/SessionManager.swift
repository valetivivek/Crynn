import Foundation
import Combine

final class SessionManager: ObservableObject {
    private let url = URL(fileURLWithPath: NSTemporaryDirectory()).appendingPathComponent("crynn_session.json")
    private var pendingSave: DispatchWorkItem?

    func restoreIfNeeded(into model: TabModel) {
        guard let data = try? Data(contentsOf: url),
              let snapshot = try? JSONDecoder().decode(Snapshot.self, from: data) else { return }
        model.tabs = snapshot.tabs
        model.activeTabID = snapshot.active
    }

    func scheduleSave(from model: TabModel) {
        pendingSave?.cancel()
        let work = DispatchWorkItem { [weak self] in self?.save(from: model) }
        pendingSave = work
        DispatchQueue.global(qos: .utility).asyncAfter(deadline: .now() + 0.6, execute: work)
    }

    private func save(from model: TabModel) {
        let snap = Snapshot(tabs: model.tabs, active: model.activeTabID)
        guard let data = try? JSONEncoder().encode(snap) else { return }
        try? data.write(to: url, options: .atomic)
    }

    private struct Snapshot: Codable { let tabs: [TabModel.Tab]; let active: UUID }
}



