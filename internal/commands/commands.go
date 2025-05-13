package commands

import (
	"errors"
	"fmt"
	"os"

	"random-picker/internal/cache"
	"random-picker/internal/config"
	"random-picker/internal/models"
	"random-picker/internal/picker"

	"github.com/spf13/cobra"
)

var groupFlag string

// SetupCommands initializes all the CLI commands
func SetupCommands() *cobra.Command {
	// Primary command
	rootCmd := &cobra.Command{
		Use:     "rp",
		Short:   "Random item picker",
		Long:    `Random item picker (rp) is a CLI tool to randomly pick items.`,
		RunE:    runPick,
		Version: "1.0.2",
	}

	// Persistent flags
	rootCmd.PersistentFlags().StringVarP(&groupFlag, "group", "g", "", "Group name")

	// Subcommands
	rootCmd.AddCommand(setupListCmd())
	rootCmd.AddCommand(setupResetCmd())
	rootCmd.AddCommand(setupCompletionCmd())
	rootCmd.AddCommand(setupGenConfigCmd())

	return rootCmd
}

// Command definitions
func setupListCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "list [type]",
		Short: "List configured items",
		Long: `List configured items based on type.

Available types:
  - all:      Lists all items in the group
  - chosen:   Lists items that have been chosen (default)
  - unchosen: Lists items that haven't been chosen yet`,
		RunE: runList,
		ValidArgsFunction: func(cmd *cobra.Command, args []string, toComplete string) ([]string, cobra.ShellCompDirective) {
			if len(args) != 0 {
				return nil, cobra.ShellCompDirectiveNoFileComp
			}
			return []string{models.TypeAll, models.TypeChosen, models.TypeUnchosen}, cobra.ShellCompDirectiveNoFileComp
		},
	}
	return cmd
}

func setupResetCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "reset",
		Short: "Reset configured items",
		RunE:  runReset,
	}
	return cmd
}

func setupCompletionCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "completion [shell]",
		Short: "Generate shell completion",
		Long: `Generate shell completion script for supported shells.
Available shells:
  - bash
  - zsh
  - fish
  - powershell`,
		Args: cobra.ExactArgs(1),
		RunE: runCompletion,
		ValidArgsFunction: func(cmd *cobra.Command, args []string, toComplete string) ([]string, cobra.ShellCompDirective) {
			if len(args) != 0 {
				return nil, cobra.ShellCompDirectiveNoFileComp
			}
			return []string{"bash", "zsh", "fish", "powershell"}, cobra.ShellCompDirectiveNoFileComp
		},
	}
	return cmd
}

func setupGenConfigCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "gen-config",
		Short: "Generate sample config",
		RunE:  runGenConfig,
	}
	return cmd
}

// Command execution functions
func runPick(cmd *cobra.Command, args []string) error {
	groupName, err := getGroupName()
	if err != nil {
		return err
	}

	group, err := config.GetGroupByName(groupName)
	if err != nil {
		return err
	}

	chosen, err := picker.ChooseRandomItem(group.Items, groupName)
	if err != nil {
		return err
	}

	fmt.Printf("Item chosen: %s\n", chosen)

	return cache.AppendChosenToCacheFile(groupName, chosen)
}

func runList(cmd *cobra.Command, args []string) error {
	listType := models.TypeChosen
	if len(args) > 0 {
		listType = args[0]
		// Validate the list type
		validTypes := []string{models.TypeAll, models.TypeChosen, models.TypeUnchosen}
		isValid := false
		for _, t := range validTypes {
			if listType == t {
				isValid = true
				break
			}
		}
		if !isValid {
			return fmt.Errorf("invalid list type: %s. Available types: %v", listType, validTypes)
		}
	}

	groupName, err := getGroupName()
	if err != nil {
		return err
	}

	group, err := config.GetGroupByName(groupName)
	if err != nil {
		return err
	}

	switch listType {
	case models.TypeAll:
		picker.PrintAllItems(groupName, group.Items)
		return nil
	case models.TypeChosen:
		return picker.PrintChosenItems(groupName)
	case models.TypeUnchosen:
		return picker.PrintUnchosenItems(group.Items, groupName)
	default:
		return fmt.Errorf("invalid list type: %s", listType)
	}
}

func runReset(cmd *cobra.Command, args []string) error {
	groupName, err := getGroupName()
	if err != nil {
		return err
	}

	return cache.ResetCache(groupName)
}

func runCompletion(cmd *cobra.Command, args []string) error {
	shell := args[0]
	rootCmd := cmd.Root()

	switch shell {
	case "bash":
		return rootCmd.GenBashCompletion(os.Stdout)
	case "zsh":
		return rootCmd.GenZshCompletion(os.Stdout)
	case "fish":
		return rootCmd.GenFishCompletion(os.Stdout, true)
	case "powershell":
		return rootCmd.GenPowerShellCompletion(os.Stdout)
	default:
		return fmt.Errorf("unsupported shell: %s", shell)
	}
}

func runGenConfig(cmd *cobra.Command, args []string) error {
	return config.GenerateDefaultConfig()
}

func getGroupName() (string, error) {
	// First check the flag
	if groupFlag != "" {
		if err := config.ParseGroup(groupFlag); err != nil {
			return "", err
		}
		return groupFlag, nil
	}

	// Then check default group
	if config.AppSetting.DefaultGroup != nil {
		return *config.AppSetting.DefaultGroup, nil
	}

	return "", errors.New("no group provided")
}
