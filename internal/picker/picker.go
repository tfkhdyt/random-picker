package picker

import (
	"errors"
	"fmt"
	"math/rand"

	"random-picker/internal/cache"
)

// ListAvailableItems returns the list of items that haven't been chosen yet
func ListAvailableItems(items []string, groupName string) ([]string, error) {
	chosenItems, err := cache.ListCacheItems(groupName)
	if err != nil {
		return nil, err
	}

	if len(items) == 0 {
		return nil, fmt.Errorf("the %s items list is empty", groupName)
	}

	var availableItems []string
	for _, item := range items {
		isChosen := false
		for _, chosen := range chosenItems {
			if item == chosen {
				isChosen = true
				break
			}
		}
		if !isChosen {
			availableItems = append(availableItems, item)
		}
	}

	return availableItems, nil
}

// ChooseRandomItem selects a random item from the available items
func ChooseRandomItem(items []string, groupName string) (string, error) {
	if len(items) == 0 {
		return "", errors.New("the items list is empty")
	}

	available, err := ListAvailableItems(items, groupName)
	if err != nil {
		return "", err
	}

	if len(available) == 0 {
		fmt.Println("All items have been chosen. Resetting the list.")
		if err := cache.ResetCache(groupName); err != nil {
			return "", err
		}
		available, err = ListAvailableItems(items, groupName)
		if err != nil {
			return "", err
		}
	}

	// Get a random item from available items
	chosen := available[rand.Intn(len(available))]

	return chosen, nil
}

// PrintChosenItems prints the list of items that have been chosen
func PrintChosenItems(groupName string) error {
	chosenItems, err := cache.ListCacheItems(groupName)
	if err != nil {
		return err
	}

	if len(chosenItems) == 0 {
		fmt.Println("No items have been chosen yet.")
	} else {
		for _, item := range chosenItems {
			fmt.Println(item)
		}
	}

	return nil
}

// PrintAllItems prints all items in a group
func PrintAllItems(groupName string, items []string) {
	if len(items) == 0 {
		fmt.Println("No items in this group is configured")
		return
	}

	for _, item := range items {
		fmt.Println(item)
	}
}

// PrintUnchosenItems prints items that haven't been chosen yet
func PrintUnchosenItems(items []string, groupName string) error {
	available, err := ListAvailableItems(items, groupName)
	if err != nil {
		return err
	}

	if len(available) == 0 {
		fmt.Println("All items have been chosen.")
	} else {
		for _, item := range available {
			fmt.Println(item)
		}
	}

	return nil
}
