import SwiftUI
import WebKit

struct SiteBlockerToggle: View {
    @EnvironmentObject private var blocker: ContentBlocker
    @EnvironmentObject private var tabs: TabModel

    var body: some View {
        let host = URL(string: tabs.activeTab?.urlString ?? "")?.host
        let enabled = blocker.isEnabled(for: host)
        return Button {
            blocker.toggleSite(host: host)
        } label: {
            Image(systemName: enabled ? "shield.lefthalf.filled" : "shield")
                .font(.system(size: 12, weight: .medium))
        }
        .buttonStyle(.plain)
        .help(enabled ? "Content blocking on this site" : "Content blocking disabled for this site")
    }
}



