#!/bin/bash

# Crynn Browser - GeckoView Installation Script
# This script installs GeckoView dependencies for building Crynn Browser

set -e

echo "🔧 Installing GeckoView dependencies for Crynn Browser..."

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🐧 Detected Linux system"
    
    # Update package manager
    if command -v apt-get &> /dev/null; then
        echo "📦 Using apt package manager"
        sudo apt-get update
        
        # Install GeckoView dependencies
        sudo apt-get install -y \
            libgeckoview-dev \
            libxul-dev \
            firefox-dev \
            pkg-config \
            build-essential \
            cmake \
            libgtk-3-dev \
            libglib2.0-dev \
            libdbus-1-dev \
            libdbus-glib-1-dev \
            libasound2-dev \
            libpulse-dev \
            libx11-dev \
            libxext-dev \
            libxrender-dev \
            libxrandr-dev \
            libxdamage-dev \
            libxfixes-dev \
            libxcomposite-dev \
            libxss-dev \
            libxtst-dev \
            libxrandr-dev \
            libgconf-2-4 \
            libgconf2-dev
            
    elif command -v yum &> /dev/null; then
        echo "📦 Using yum package manager"
        sudo yum update -y
        
        # Install GeckoView dependencies
        sudo yum install -y \
            geckoview-devel \
            firefox-devel \
            pkgconfig \
            gcc-c++ \
            cmake \
            gtk3-devel \
            glib2-devel \
            dbus-devel \
            dbus-glib-devel \
            alsa-lib-devel \
            pulseaudio-libs-devel \
            libX11-devel \
            libXext-devel \
            libXrender-devel \
            libXrandr-devel \
            libXdamage-devel \
            libXfixes-devel \
            libXcomposite-devel \
            libXScrnSaver-devel \
            libXtst-devel \
            GConf2-devel
            
    elif command -v pacman &> /dev/null; then
        echo "📦 Using pacman package manager"
        sudo pacman -Sy
        
        # Install GeckoView dependencies
        sudo pacman -S --needed \
            geckoview \
            firefox \
            pkg-config \
            base-devel \
            cmake \
            gtk3 \
            glib2 \
            dbus \
            dbus-glib \
            alsa-lib \
            libpulse \
            libx11 \
            libxext \
            libxrender \
            libxrandr \
            libxdamage \
            libxfixes \
            libxcomposite \
            libxss \
            libxtst
            
    else
        echo "❌ Unsupported package manager. Please install GeckoView manually."
        exit 1
    fi
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 Detected macOS system"
    
    # Check for Homebrew
    if ! command -v brew &> /dev/null; then
        echo "📦 Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    
    # Install GeckoView dependencies
    echo "📦 Installing GeckoView dependencies..."
    brew install \
        geckoview \
        firefox \
        pkg-config \
        cmake \
        gtk+3 \
        glib \
        dbus \
        alsa-lib \
        pulseaudio \
        libx11 \
        libxext \
        libxrender \
        libxrandr \
        libxdamage \
        libxfixes \
        libxcomposite
    
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    echo "🪟 Detected Windows system"
    
    # Check for vcpkg
    if ! command -v vcpkg &> /dev/null; then
        echo "📦 Installing vcpkg..."
        git clone https://github.com/Microsoft/vcpkg.git
        cd vcpkg
        ./bootstrap-vcpkg.sh
        ./vcpkg integrate install
        cd ..
    fi
    
    # Install GeckoView dependencies
    echo "📦 Installing GeckoView dependencies..."
    vcpkg install \
        geckoview \
        firefox \
        pkg-config \
        cmake \
        gtk \
        glib \
        dbus \
        alsa \
        pulseaudio
    
else
    echo "❌ Unsupported operating system: $OSTYPE"
    exit 1
fi

echo "✅ GeckoView dependencies installed successfully!"
echo ""
echo "🚀 You can now build Crynn Browser with:"
echo "   cargo build --release"
echo ""
echo "📺 To test YouTube support, run:"
echo "   cargo run --release"
echo "   Then navigate to https://youtube.com"
