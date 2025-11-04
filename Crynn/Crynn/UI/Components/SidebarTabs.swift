import SwiftUI

struct SidebarTabs: View {
    @EnvironmentObject private var tabs: TabModel

    var body: some View {
        VStack(spacing: 0) {
            HStack {
                Text("Tabs").font(.caption).foregroundStyle(.secondary)
                Spacer()
                Button { tabs.newTab() } label: { Image(systemName: "plus") }
                    .buttonStyle(.plain)
            }
            .padding(8)
            List(selection: Binding(get: { tabs.activeTabID }, set: { tabs.activeTabID = $0 })) {
                ForEach(tabs.tabs) { tab in
                    HStack {
                        if tab.isPinned { Image(systemName: "pin.fill") }
                        VStack(alignment: .leading) {
                            Text(tab.title.isEmpty ? "New Tab" : tab.title).lineLimit(1)
                            Text(tab.urlString).font(.caption2).foregroundStyle(.secondary).lineLimit(1)
                        }
                        Spacer()
                    }
                    .tag(tab.id)
                    .contextMenu {
                        Button("Duplicate") { tabs.activeTabID = tab.id; tabs.duplicateActive() }
                        Button(tab.isPinned ? "Unpin" : "Pin") { tabs.togglePinned(for: tab.id) }
                        Button("Close") { if tabs.activeTabID == tab.id { tabs.closeActiveTab() } else if let idx = tabs.tabs.firstIndex(of: tab) { tabs.tabs.remove(at: idx) } }
                    }
                }
            }
            .listStyle(.inset)
        }
    }
}

