import SwiftUI

struct Omnibox: View {
    @EnvironmentObject private var tabs: TabModel
    @EnvironmentObject private var nav: NavigationState

    var body: some View {
        TextField("Search or enter address", text: Binding(
            get: { tabs.activeTab?.urlString ?? "" },
            set: { 
                if let id = tabs.activeTab?.id { 
                    tabs.setURL($0, for: id)
                }
            }
        ))
        .textFieldStyle(.plain)
        .padding(.horizontal, 10)
        .padding(.vertical, 4)
        .background(.regularMaterial, in: RoundedRectangle(cornerRadius: 6))
        .font(.system(size: 13))
        .onSubmit {
            // Trigger navigation update
            if let id = tabs.activeTab?.id, let urlString = tabs.activeTab?.urlString {
                tabs.setURL(urlString, for: id)
            }
        }
    }
}



