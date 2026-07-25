import AppKit
import Darwin
import Foundation

final class AppDelegate: NSObject, NSApplicationDelegate {
    private var backend: Process?
    private let port = ProcessInfo.processInfo.environment["PANCANGA_RC1_PORT"] ?? "7979"

    func applicationDidFinishLaunching(_ notification: Notification) {
        do {
            try startBackend()
            NSWorkspace.shared.open(URL(string: "http://127.0.0.1:\(port)/")!)
        } catch {
            showStartupError(error)
            NSApp.terminate(nil)
        }
    }

    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        stopBackend()
        return .terminateNow
    }

    private func startBackend() throws {
        guard let resources = Bundle.main.resourceURL else {
            throw LauncherError.missingResourceDirectory
        }

        let backendURL = resources.appendingPathComponent("rc1_experience")
        let contentURL = resources.appendingPathComponent("content/ekadasi")
        let logDirectory = URL(fileURLWithPath: "/tmp/PancangaEngine", isDirectory: true)
        let logURL = logDirectory.appendingPathComponent("rc1.log")

        try FileManager.default.createDirectory(
            at: logDirectory,
            withIntermediateDirectories: true
        )
        FileManager.default.createFile(atPath: logURL.path, contents: nil)
        let logHandle = try FileHandle(forWritingTo: logURL)

        let process = Process()
        process.executableURL = backendURL
        var environment = ProcessInfo.processInfo.environment
        environment["RC1_EXPERIENCE_ADDRESS"] = "127.0.0.1:\(port)"
        environment["RC1_CONTENT_DIR"] = contentURL.path
        environment["RC1_COMMIT"] = "packaged-rc1"
        process.environment = environment
        process.standardOutput = logHandle
        process.standardError = logHandle

        try process.run()
        backend = process

        try waitUntilBackendIsOnline()
    }

    private func waitUntilBackendIsOnline() throws {
        for _ in 0..<80 {
            if isBackendOnline() {
                return
            }
            if backend?.isRunning == false {
                throw LauncherError.backendExited
            }
            Thread.sleep(forTimeInterval: 0.25)
        }
        throw LauncherError.backendTimeout
    }

    private func isBackendOnline() -> Bool {
        let check = Process()
        check.executableURL = URL(fileURLWithPath: "/usr/bin/curl")
        check.arguments = [
            "-fsS",
            "http://127.0.0.1:\(port)/rc1-config.js",
            "-o",
            "/dev/null",
        ]
        check.standardOutput = FileHandle.nullDevice
        check.standardError = FileHandle.nullDevice

        do {
            try check.run()
            check.waitUntilExit()
            return check.terminationStatus == 0
        } catch {
            return false
        }
    }

    private func stopBackend() {
        guard let backend else {
            return
        }

        if backend.isRunning {
            backend.terminate()
            for _ in 0..<20 {
                if !backend.isRunning {
                    break
                }
                Thread.sleep(forTimeInterval: 0.1)
            }
        }

        if backend.isRunning {
            kill(backend.processIdentifier, SIGKILL)
        }
        self.backend = nil
    }

    private func showStartupError(_ error: Error) {
        let alert = NSAlert()
        alert.messageText = "Pancanga Engine RC1 no pudo iniciar"
        alert.informativeText = "\(error)"
        alert.alertStyle = .critical
        alert.runModal()
    }
}

enum LauncherError: Error {
    case missingResourceDirectory
    case backendExited
    case backendTimeout
}

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate
app.setActivationPolicy(.regular)
app.run()
