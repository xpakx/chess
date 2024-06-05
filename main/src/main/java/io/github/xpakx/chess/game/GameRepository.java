package io.github.xpakx.chess.game;

import org.springframework.data.jpa.repository.EntityGraph;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;

import java.util.List;
import java.util.Optional;

public interface GameRepository extends JpaRepository<Game, Long> {
    @Query("select g from Game g " +
            "left join fetch g.user " +
            "left join fetch g.opponent " +
            "where g.opponent.id = ?1 and g.invitation = io.github.xpakx.chess.game.InvitationStatus.Issued")
    List<Game> findRequests(Long id);

    @Query("select g from Game g " +
            "left join fetch g.user " +
            "left join fetch g.opponent " +
            "where " +
            "(g.user.id = ?1 or g.opponent.id = ?1) " +
            "and g.invitation = io.github.xpakx.chess.game.InvitationStatus.Accepted " +
            "and g.status = io.github.xpakx.chess.game.GameStatus.NotFinished")
    List<Game> findActiveGames(Long id);

    @Query("select g from Game g " +
            "left join fetch g.user " +
            "left join fetch g.opponent " +
            "where " +
            "(g.user.id = ?1 or g.opponent.id = ?1) " +
            "and g.invitation = io.github.xpakx.chess.game.InvitationStatus.Accepted " +
            "and g.status != io.github.xpakx.chess.game.GameStatus.NotFinished")
    List<Game> findFinishedGames(Long id);

    @EntityGraph(attributePaths = {"user", "opponent"})
    Optional<Game> findWithUsersById(Long id);

    @EntityGraph(attributePaths = {"opponent"})
    Optional<Game> findWithOpponentById(Long id);
}
