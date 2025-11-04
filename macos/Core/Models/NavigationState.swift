import Foundation
import Combine

final class NavigationState: ObservableObject {
    @Published var omniboxText: String = ""
    @Published var focusOmnibox: Bool = false
    @Published var quickSwitch: Bool = false
}



