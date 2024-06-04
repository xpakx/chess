package io.github.xpakx.chess.game;

import com.fasterxml.jackson.annotation.JsonIgnore;
import io.github.xpakx.chess.user.User;
import jakarta.persistence.*;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.time.LocalDateTime;

@Entity
@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
public class Game {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    private InvitationStatus invitation;
    private GameType type;
    private AIType aiType;
    private GameStatus status;

    @Column(columnDefinition = "TIMESTAMP")
    private LocalDateTime startedAt;
    @Column(columnDefinition = "TIMESTAMP")
    private LocalDateTime lastMoveAt;

    private String currentState;
    private Integer nonCaptureMoves;

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id", nullable = false)
    @JsonIgnore
    private User user;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "opponent_id")
    @JsonIgnore
    private User opponent;

    private boolean userStarts;
    private boolean userTurn;
}
