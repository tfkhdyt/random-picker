package cache

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// GetAppCacheDir returns the cache directory for the application
func GetAppCacheDir() string {
	cacheDir, err := os.UserCacheDir()
	if err != nil {
		panic(fmt.Sprintf("Could not determine user cache directory: %v", err))
	}
	return filepath.Join(cacheDir, "random-picker")
}

// GetCacheFilePath returns the path to a group's cache file
func GetCacheFilePath(groupName string) string {
	return filepath.Join(GetAppCacheDir(), fmt.Sprintf("%s.txt", groupName))
}

// ListCacheItems returns the list of chosen items for a group
func ListCacheItems(groupName string) ([]string, error) {
	cacheFile := GetCacheFilePath(groupName)

	// If file doesn't exist, return empty list
	if _, err := os.Stat(cacheFile); os.IsNotExist(err) {
		return []string{}, nil
	}

	file, err := os.Open(cacheFile)
	if err != nil {
		return nil, fmt.Errorf("failed to open cache file: %w", err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading cache file: %w", err)
	}

	return lines, nil
}

// AppendChosenToCacheFile adds a chosen item to the cache file
func AppendChosenToCacheFile(groupName, chosen string) error {
	cacheFile := GetCacheFilePath(groupName)
	cacheDir := filepath.Dir(cacheFile)

	// Create cache directory if it doesn't exist
	if err := os.MkdirAll(cacheDir, 0o755); err != nil {
		return fmt.Errorf("failed to create cache directory: %w", err)
	}

	file, err := os.OpenFile(cacheFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return fmt.Errorf("failed to open cache file: %w", err)
	}
	defer file.Close()

	if _, err := file.WriteString(chosen + "\n"); err != nil {
		return fmt.Errorf("failed to write to cache file: %w", err)
	}

	return nil
}

// ResetCache resets the cache for a group, keeping only the last 2 chosen items
func ResetCache(groupName string) error {
	cacheFile := GetCacheFilePath(groupName)

	// If file doesn't exist, nothing to reset
	if _, err := os.Stat(cacheFile); os.IsNotExist(err) {
		return nil
	}

	// Read the file content
	content, err := os.ReadFile(cacheFile)
	if err != nil {
		return fmt.Errorf("failed to read cache file: %w", err)
	}

	lines := strings.Split(string(content), "\n")
	// Remove empty lines
	var nonEmptyLines []string
	for _, line := range lines {
		if line != "" {
			nonEmptyLines = append(nonEmptyLines, line)
		}
	}

	// Keep only the last 2 lines if there are at least 2 lines
	var linesToKeep []string
	if len(nonEmptyLines) >= 2 {
		linesToKeep = nonEmptyLines[len(nonEmptyLines)-2:]
	}

	// Write back
	contentToWrite := strings.Join(linesToKeep, "\n")
	if len(linesToKeep) > 0 {
		contentToWrite += "\n"
	}

	if err := os.WriteFile(cacheFile, []byte(contentToWrite), 0o644); err != nil {
		return fmt.Errorf("failed to write to cache file: %w", err)
	}

	fmt.Println("Chosen items list has been reset.")
	return nil
}

// EnsureCacheDirExists ensures the cache directory exists
func EnsureCacheDirExists() error {
	return os.MkdirAll(GetAppCacheDir(), 0o755)
}
