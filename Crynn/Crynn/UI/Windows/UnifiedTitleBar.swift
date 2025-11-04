import SwiftUI

struct UnifiedTitleBar: View {
    @EnvironmentObject private var tabs: TabModel
    @EnvironmentObject private var nav: NavigationState

    var body: some View {
        HStack(spacing: 6) {
            // Minimal navigation controls
            HStack(spacing: 4) {
                Button {
                    // Back navigation
                } label: {
                    Image(systemName: "chevron.left")
                        .font(.system(size: 12, weight: .medium))
                }
                .buttonStyle(.plain)
                .disabled(true)
                Button {
                    // Forward navigation
                } label: {
                    Image(systemName: "chevron.right")
                        .font(.system(size: 12, weight: .medium))
                }
                .buttonStyle(.plain)
                .disabled(true)
            }
            .foregroundStyle(.secondary)

            // Compact omnibox
            Omnibox()
                .frame(maxWidth: .infinity)

            // Minimal actions
            HStack(spacing: 4) {
                Button {
                    if let id = tabs.activeTab?.id { tabs.toggleReader(for: id) }
                } label: {
                    Image(systemName: (tabs.activeTab?.isReader ?? false) ? "doc.text.viewfinder" : "doc.text")
                        .font(.system(size: 12, weight: .medium))
                }
                .buttonStyle(.plain)
                SiteBlockerToggle()
            }
            .foregroundStyle(.secondary)
        }
    }
}



