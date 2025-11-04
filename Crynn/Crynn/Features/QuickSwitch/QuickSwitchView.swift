import SwiftUI

struct QuickSwitchView: View {
    @EnvironmentObject private var tabs: TabModel
    @EnvironmentObject private var nav: NavigationState
    @State private var query: String = ""

    private var filtered: [TabModel.Tab] {
        guard !query.isEmpty else { return tabs.tabs }
        let q = query.lowercased()
        return tabs.tabs.filter { $0.title.lowercased().contains(q) || $0.urlString.lowercased().contains(q) }
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            TextField("Quick Switch…", text: $query)
                .textFieldStyle(.roundedBorder)
            List(filtered, id: \.id) { tab in
                HStack {
                    if tab.isPinned { Image(systemName: "pin.fill") }
                    VStack(alignment: .leading) {
                        Text(tab.title.isEmpty ? "New Tab" : tab.title).lineLimit(1)
                        Text(tab.urlString).font(.caption2).foregroundStyle(.secondary).lineLimit(1)
                    }
                    Spacer()
                }
                .contentShape(Rectangle())
                .onTapGesture {
                    tabs.activeTabID = tab.id
                    nav.quickSwitch = false
                }
            }
            .frame(width: 520, height: 360)
        }
        .padding(16)
    }
}



