-- DROP DATABASE IF EXISTS tournament_manager;
CREATE DATABASE IF NOT EXISTS tournament_manager;

USE tournament_manager;

CREATE TABLE `users` (
	`id` BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `email` VARCHAR(255) NOT NULL,
    `password` VARCHAR(255) NOT NULL,
    `name` VARCHAR(255) NOT NULL
);

CREATE TABLE `tournaments` (
	`id` BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(20) NOT NULL,
    `public` BOOLEAN NOT NULL DEFAULT FALSE,
    
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) 
		ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE `leagues` (
	`id` BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `tournament_id` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(20) NOT NULL,
    `completed` BOOLEAN NOT NULL DEFAULT FALSE,
    
    FOREIGN KEY (`tournament_id`) REFERENCES `tournaments`(`id`)
		ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE `teams` (
	`id` BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `tournament_id` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(40) NOT NULL,
    
    FOREIGN KEY (`tournament_id`) REFERENCES `tournaments`(`id`)
		ON DELETE CASCADE ON UPDATE CASCADE
);


CREATE TABLE `fixtures` (
	`id` BIGINT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `home_team_id` BIGINT UNSIGNED NOT NULL,
    `away_team_id` BIGINT UNSIGNED NOT NULL,
    `league_id` BIGINT UNSIGNED NOT NULL,
    `playing_date` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `home_score` TINYINT UNSIGNED NOT NULL DEFAULT 0,
    `away_score`  TINYINT UNSIGNED NOT NULL DEFAULT 0,
    `played` BOOLEAN NOT NULL DEFAULT FALSE,
    `round` SMALLINT UNSIGNED NOT NULL,
    
    FOREIGN KEY (`home_team_id`) REFERENCES `teams`(`id`)
		ON UPDATE CASCADE,
	
    FOREIGN KEY (`away_team_id`) REFERENCES `teams`(`id`)
		ON UPDATE CASCADE,
        
	FOREIGN KEY (`league_id`) REFERENCES `leagues`(`id`)
		ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE `teams_leagues` (
	`team_id` BIGINT UNSIGNED NOT NULL,
    `league_id` BIGINT UNSIGNED NOT NULL,
    
    PRIMARY KEY(`team_id`, `league_id`),
    
    FOREIGN KEY (`team_id`) REFERENCES `teams`(`id`)
		ON UPDATE CASCADE,
	
    FOREIGN KEY (`league_id`) REFERENCES `leagues`(`id`)
		ON DELETE CASCADE ON UPDATE CASCADE
);

DELIMITER //
CREATE PROCEDURE generate_fixture(
	IN home_team_id BIGINT UNSIGNED,
    IN away_team_id BIGINT UNSIGNED,
    IN league_id BIGINT UNSIGNED,
    IN round TINYINT UNSIGNED,
    IN user_id BIGINT UNSIGNED
)
BEGIN
	DECLARE tournament_exists INT;
    DECLARE home_team_exists INT;
    DECLARE away_team_exists INT;
    DECLARE league_exists INT;
    DECLARE actual_tournament_id BIGINT UNSIGNED;
    
    SELECT COUNT(*), TournamentRow.id INTO tournament_exists, actual_tournament_id
    FROM `tournaments` as TournamentRow
    INNER JOIN `leagues` as LeagueRow
		ON LeagueRow.id = league_id AND LeagueRow.tournament_id = TournamentRow.id
	WHERE TournamentRow.user_id = user_id
    GROUP BY TournamentRow.id;
    
    IF tournament_exists = 0 THEN
		SIGNAL SQLSTATE '45000'
		SET MESSAGE_TEXT = 'Tournament not found';
    END IF;
    
    SELECT COUNT(*) INTO league_exists
    FROM `leagues` as LeagueRow
    WHERE LeagueRow.id = league_id;
    
    IF league_exists = 0 THEN
		SIGNAL SQLSTATE '45000'
		SET MESSAGE_TEXT = 'League not found';
    END IF;
    
    SELECT COUNT(*) INTO home_team_exists
    FROM `teams` as TeamRow
    WHERE TeamRow.id = home_team_id AND TeamRow.tournament_id = actual_tournament_id;

    IF home_team_exists = 0 THEN
		SIGNAL SQLSTATE '45000'
		SET MESSAGE_TEXT = 'Home team not found on the tournament';
    END IF;
    
    SELECT COUNT(*) INTO away_team_exists
    FROM `teams` as TeamRow
    WHERE TeamRow.id = away_team_id AND TeamRow.tournament_id = actual_tournament_id;
    
    IF away_team_exists = 0 THEN
		SIGNAL SQLSTATE '45000'
		SET MESSAGE_TEXT = 'Away team not found on the tournament';
    END IF;
    
    INSERT INTO `fixtures` (home_team_id, away_team_id, league_id, round)
    VALUES (home_team_id, away_team_id, league_id, round);
END //
DELIMITER ;
