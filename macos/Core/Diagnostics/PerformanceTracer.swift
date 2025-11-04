import Foundation
import os.signpost

final class PerformanceTracer {
    static let shared = PerformanceTracer()
    private let log = OSLog(subsystem: "dev.crynn.browser", category: "perf")
    private var startTimes: [UUID: UInt64] = [:]

    func appLaunched() {
        os_signpost(.begin, log: log, name: "AppLaunch")
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
            os_signpost(.end, log: self.log, name: "AppLaunch")
        }
    }

    func pageRequested(tabId: UUID) {
        startTimes[tabId] = mach_absolute_time()
        os_signpost(.begin, log: log, name: "PageLoad", signpostID: .init(UInt32(truncatingIfNeeded: tabId.hashValue)))
    }

    func pageStarted(tabId: UUID) {
        os_signpost(.event, log: log, name: "Provisional", signpostID: .init(UInt32(truncatingIfNeeded: tabId.hashValue)))
    }

    func pageFinished(tabId: UUID) {
        os_signpost(.end, log: log, name: "PageLoad", signpostID: .init(UInt32(truncatingIfNeeded: tabId.hashValue)))
        startTimes.removeValue(forKey: tabId)
    }

    func tabCreated(id: UUID) {
        os_signpost(.event, log: log, name: "TabCreated", signpostID: .init(UInt32(truncatingIfNeeded: id.hashValue)))
    }

    func tabClosed(id: UUID) {
        os_signpost(.event, log: log, name: "TabClosed", signpostID: .init(UInt32(truncatingIfNeeded: id.hashValue)))
    }
}



