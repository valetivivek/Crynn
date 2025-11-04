import SwiftUI

struct BrowserWindow: View {
    @EnvironmentObject private var tabs: TabModel
    @EnvironmentObject private var nav: NavigationState

    var body: some View {
        HStack(spacing: 0) {
            SidebarTabs()
                .frame(width: 180)
                .background(.ultraThinMaterial)

            VStack(spacing: 0) {
                UnifiedTitleBar()
                    .padding(.horizontal, 12)
                    .padding(.vertical, 4)
                    .background(.ultraThinMaterial)
                Divider()
                ZStack {
                    if let active = tabs.activeTab {
                        WebViewContainer(
                            urlString: Binding(get: { active.urlString }, set: { tabs.setURL($0, for: active.id) }),
                            isReader: Binding(get: { active.isReader }, set: { _ in /* toggled externally */ }),
                            tabId: active.id
                        )
                        .id(active.id)
                    } else {
                        Color.clear
                    }
                }
            }
        }
        .frame(minWidth: 900, minHeight: 600)
        .sheet(isPresented: $nav.quickSwitch) { QuickSwitchView() }
    }
}



