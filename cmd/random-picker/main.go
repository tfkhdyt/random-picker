package main

import (
	"fmt"
	"math/rand"
	"os"
	"time"

	"random-picker/internal/cache"
	"random-picker/internal/commands"
	"random-picker/internal/config"
	"random-picker/internal/models"
)

func init() {
	// Set up the rand seed for older Go versions if needed
	// In Go 1.20+ this is no longer necessary as the math/rand
	// package is now automatically seeded with a secure, random value
	// but we'll keep it for compatibility
	rand.Seed(time.Now().UnixNano())
}

// Execute runs the application
func Execute() {
	// Create app cache directory
	if err := cache.EnsureCacheDirExists(); err != nil {
		fmt.Printf("Failed to create cache directory: %v\n", err)
		os.Exit(1)
	}

	// Try loading app settings
	configPath := config.GetConfigPath()
	if _, err := os.Stat(configPath); !os.IsNotExist(err) {
		if err := config.LoadAppSettings(); err != nil {
			fmt.Printf("Failed to load settings: %v\n", err)
			os.Exit(1)
		}
	} else {
		// Initialize empty settings if no config exists
		config.AppSetting = &models.AppSetting{
			Groups: []models.Group{},
		}
	}

	// Set up and execute the root command
	rootCmd := commands.SetupCommands()
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func main() {
	Execute()
}
