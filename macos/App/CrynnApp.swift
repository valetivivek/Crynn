import SwiftUI
import WebKit
import os.signpost

@main
struct CrynnApp: App {
    @StateObject private var tabModel = TabModel()
    @StateObject private var navigation = NavigationState()
    @StateObject private var session = SessionManager()
    @StateObject private var blocker = ContentBlocker()

    var body: some Scene {
        WindowGroup("Crynn") {
            BrowserWindow()
                .environmentObject(tabModel)
                .environmentObject(navigation)
                .environmentObject(session)
                .environmentObject(blocker)
                .onAppear {
                    PerformanceTracer.shared.appLaunched()
                    session.restoreIfNeeded(into: tabModel)
                }
                .onChange(of: tabModel.tabs) { _ in
                    session.scheduleSave(from: tabModel)
                }
        }
        .commands {
            CommandGroup(replacing: .newItem) {
                Button("New Tab", action: { tabModel.newTab() })
                    .keyboardShortcut("t", modifiers: .command)
                Button("Close Tab", action: { tabModel.closeActiveTab() })
                    .keyboardShortcut("w", modifiers: .command)
            }
            CommandMenu("Navigation") {
                Button("Focus Omnibox") { navigation.focusOmnibox.toggle() }
                    .keyboardShortcut("l", modifiers: .command)
                Button("Quick Switch") { navigation.quickSwitch.toggle() }
                    .keyboardShortcut("k", modifiers: .command)
            }
        }
        Settings {
            SettingsView()
                .environmentObject(blocker)
        }
    }
}


