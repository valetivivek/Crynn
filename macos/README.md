# Crynn macOS App - Xcode Setup

## Quick Setup in Xcode

1. **Create New Project** (if you haven't already):
   - File → New → Project
   - macOS → App
   - Product Name: `Crynn`
   - Organization Identifier: `dev.crynn`
   - Interface: SwiftUI
   - Language: Swift

2. **Add Files to Target**:
   - Select all Swift files in the `macos/` folder
   - Right-click → "Add Files to Crynn..."
   - **IMPORTANT**: In the dialog that appears:
     - Check "Copy items if needed" (or leave unchecked if files are already in project location)
     - **Target Membership**: Make sure "Crynn" is checked (not "engine" or any other target)
     - Click "Add"

3. **Verify Target Membership**:
   - Select each file in the Project Navigator
   - In the File Inspector (right panel), under "Target Membership"
   - Ensure "Crynn" is checked for all files

4. **Build Settings**:
   - Select the "Crynn" target
   - Deployment Target: macOS 13.0 or later
   - Ensure Swift files are in "Compile Sources" build phase

## File Structure

All files are organized in the `macos/` directory. Make sure all Swift files are added to the "Crynn" target, not "engine".

