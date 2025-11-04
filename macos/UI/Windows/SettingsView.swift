import SwiftUI

struct SettingsView: View {
    @EnvironmentObject private var blocker: ContentBlocker
    @StateObject private var settings = SettingsStore()

    var body: some View {
        Form {
            Toggle("Enable Content Blocking", isOn: $blocker.enabled)
            Button("Clear Website Data") { blocker.clearAllWebsiteData() }
            Picker("Search Engine", selection: $settings.searchEngine) {
                ForEach(SettingsStore.SearchEngine.allCases) { e in
                    Text(e.rawValue.capitalized).tag(e)
                }
            }
            TextField("Homepage", text: $settings.homepage)
        }.padding(16)
        .frame(width: 420)
    }
}

