package config

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"

	"random-picker/internal/models"

	"gopkg.in/yaml.v3"
)

var AppSetting *models.AppSetting

// GetAppConfigDir returns the configuration directory for the application
func GetAppConfigDir() string {
	configDir, err := os.UserConfigDir()
	if err != nil {
		panic(fmt.Sprintf("Could not determine user config directory: %v", err))
	}
	return filepath.Join(configDir, "random-picker")
}

// GetConfigPath returns the path to the configuration file
func GetConfigPath() string {
	return filepath.Join(GetAppConfigDir(), "config.yaml")
}

// LoadAppSettings loads the application settings from the config file
func LoadAppSettings() error {
	configData, err := os.ReadFile(GetConfigPath())
	if err != nil {
		return fmt.Errorf("failed to read config file: %w", err)
	}

	AppSetting = &models.AppSetting{}
	err = yaml.Unmarshal(configData, AppSetting)
	if err != nil {
		return fmt.Errorf("failed to parse config file: %w", err)
	}

	return nil
}

// WriteToConfig writes configuration data to the config file
func WriteToConfig(newConfig string) error {
	configPath := GetConfigPath()
	configDir := filepath.Dir(configPath)

	// Create config directory if it doesn't exist
	if err := os.MkdirAll(configDir, 0o755); err != nil {
		return fmt.Errorf("failed to create config directory: %w", err)
	}

	if err := os.WriteFile(configPath, []byte(newConfig), 0o644); err != nil {
		return fmt.Errorf("failed to write to config: %w", err)
	}

	return LoadAppSettings()
}

// GetGroupNames returns all the group names from the configuration
func GetGroupNames() []string {
	var names []string
	for _, group := range AppSetting.Groups {
		names = append(names, group.Name)
	}
	return names
}

// ParseGroup checks if a group name is valid
func ParseGroup(s string) error {
	groupNames := GetGroupNames()
	for _, name := range groupNames {
		if name == s {
			return nil
		}
	}
	return fmt.Errorf("invalid group. Valid options: %v", groupNames)
}

// GetGroupByName returns a group by its name
func GetGroupByName(groupName string) (*models.Group, error) {
	for _, g := range AppSetting.Groups {
		if g.Name == groupName {
			return &g, nil
		}
	}
	return nil, fmt.Errorf("group not found: %s", groupName)
}

// GenerateDefaultConfig creates a default configuration file
func GenerateDefaultConfig() error {
	configPath := GetConfigPath()

	// Check if config already exists
	if _, err := os.Stat(configPath); err == nil {
		return errors.New("config file already exists")
	}

	// Create default config
	defaultSetting := models.AppSetting{
		DefaultGroup: models.StrPtr("games"),
		Groups: []models.Group{
			{
				Name:  "games",
				Items: []string{"Balatro", "Marvel Rivals"},
			},
		},
	}

	yamlData, err := yaml.Marshal(&defaultSetting)
	if err != nil {
		return fmt.Errorf("failed to serialize config: %w", err)
	}

	if err := WriteToConfig(string(yamlData)); err != nil {
		return err
	}

	fmt.Println("Default config file has been generated!")
	return nil
}
